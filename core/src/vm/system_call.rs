use wasmi::*;
use wasmi::ValueType::*;

use std::sync::Mutex;
use std::collections::HashMap;

use super::kernel::Kernel;
use super::selector::Selector;
use super::runtime::RuntimeResult;

pub const RETURN_INDEX: usize       = 0x01;
pub const CALL_INDEX:   usize       = 0x02;
pub const CREATE_INDEX: usize       = 0x03;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

macro_rules! void {
	{ $e: expr } => { { Ok(None) } }
}

macro_rules! some {
	{ $e: expr } => { { Ok(Some($e)) } }
}

macro_rules! cast {
	{ $e: expr } => { { Ok(Some($e)) } }
}

pub trait Api {
    fn call(&self, addr: u32, abi: u32);
}

pub struct SystemCall<'a> {
    kernel: Option<&'a mut Kernel>
}

impl <'a> SystemCall<'a> {
    pub fn new() -> Self {
        SystemCall {
            kernel: None
        }
    }

    pub fn init_with_kernel(&'a mut self, kernel: &'a mut Kernel){
        self.kernel = Some(kernel);
    }

}

impl <'a> Api for SystemCall<'a> {
    fn call(&self, addr: u32, abi: u32) {
        println!("test123");
    }
}

pub trait SysCallRegister {
    fn register(&self, sys_resolver: &SysCallResolver) -> ModuleRef;
}

impl SysCallRegister for Module {
    fn register(&self, sys_resolver: &SysCallResolver) -> ModuleRef {
        let mut imports = ImportsBuilder::new();
        imports.push_resolver("env", sys_resolver);

        ModuleInstance::new(
            self,
            &imports,
        ).expect("Failed to instantiate module")
            .assert_no_start()
    }
}

impl <'a> Externals for SystemCall<'a> {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            CALL_INDEX => void!(self.call(args)),
            _ => panic!("unknown function index {}", index)
        }
    }
}

/// # SysCallResolver
/// **Usage**
/// - WASMI Import resolver
/// **Member**
/// - 1. ***max_memory***:      instance of [u32]
#[derive(Default)]
pub struct SysCallResolver {
    system_call_table: HashMap<usize, Signature>
}

impl SysCallResolver {
    /// # new(2)
    /// **Usage**
    /// - create new SysCallResolver
    /// **Return**
    /// - 1. ***Self***
    /// ## Examples
    /// ```
    /// ```
    pub fn new(max_memory: u32) -> SysCallResolver {
        SysCallResolver {
            system_call_table: hashmap![
                CALL_INDEX => Signature::new(&[I32, I32][..], None)
            ]
        }
    }

    /// # func_ref
    /// **Usage**
    /// - Generate FuncRef for WASMI based on external functions
    /// **Parameters**
    /// - 1. ***usize(index)***: external function index
    /// **Return**
    /// - 1. ***Option<FuncRef>***
    /// ## Examples
    /// ```
    /// ```
    pub fn func_ref(&self, index: usize) -> Option<FuncRef> {
        self.system_call_table.get(&index).and_then(|sign| {
            Some(FuncInstance::alloc_host(sign.to_owned(), index))
        })
    }
}

impl ModuleImportResolver for SysCallResolver {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, Error> {
        match field_name {
            "_Z4callPcS_" => {
                match self.func_ref(CALL_INDEX) {
                    Some(f) => Ok(f),
                    None => Err(Error::Function(
                        format!("function index: {} is not register in the kernel", CALL_INDEX)
                    ))
                }
            },
            _ =>
                Err(Error::Function(
                    format!("kernel module doesn't export function with name {}", field_name)
                ))
        }
    }
}
