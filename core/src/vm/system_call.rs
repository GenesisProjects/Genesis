use wasmi::*;
use wasmi::ValueType::*;
use std::sync::Mutex;

use std::collections::HashMap;

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
    fn create() {
        unimplemented!()
    }

    fn call() {
        unimplemented!()
    }

    fn ret() {
        unimplemented!()
    }
}

pub struct SystemCall {
    system_call_table: HashMap<usize, Signature>
}

impl SystemCall {
    pub fn new() -> Self {
        SystemCall {
            system_call_table: hashmap![
                CALL_INDEX => Signature::new(&[I32, I32][..], Some(I32))
            ]
        }
    }

    pub fn func_ref(&self, index: usize) -> Option<FuncRef> {
       self.system_call_table.get(&index).and_then(|sign| {
           Some(FuncInstance::alloc_host(sign.to_owned(), index))
       })
    }
}

impl Api for SystemCall {

}

pub trait SysCallRegister {
    fn register(&self, kernel: &SystemCall) -> ModuleRef;
}

impl SysCallRegister for Module {
    fn register(&self, sys_call: &SystemCall) -> ModuleRef {
        let mut imports = ImportsBuilder::new();
        imports.push_resolver("sys_call", sys_call);

        ModuleInstance::new(
            self,
            &imports,
        ).expect("Failed to instantiate module")
            .assert_no_start()
    }
}

impl Externals for SystemCall {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            CALL_INDEX => void!(SystemCall::call()),
            _ => panic!("unknown function index {}", index)
        }
    }
}

impl ModuleImportResolver for SystemCall {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, Error> {
        match field_name {
            "call" => {
                match SYSTEM_CALL.lock().unwrap().func_ref(CALL_INDEX) {
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

    fn resolve_memory(
        &self,
        field_name: &str,
        descriptor: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        if field_name == "memory" {
            unimplemented!()
        } else {
            Err(Error::Instantiation("Memory imported under unknown name".to_owned()))
        }
    }
}
