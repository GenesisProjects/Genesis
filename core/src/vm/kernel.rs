use std::cell::RefCell;
use std::collections::HashMap;

use wasmi::*;

use super::selector::Selector;
use super::gen_vm::GenVM;
use super::system_call::*;
use super::runtime::*;
use common::address::Address;
use common::hash::Hash;
use action::Action;
use account::Account;

pub type CHUNK = [u8; 32];

macro_rules! void {
	{ $e: expr } => { { Ok(None) } }
}

macro_rules! some {
	{ $e: expr } => { { Ok(Some($e)) } }
}

macro_rules! cast {
	{ $e: expr } => { { Ok(Some($e)) } }
}

#[derive(Clone)]
pub struct KernelCache {
    memory: HashMap<Hash, CHUNK>
}

impl KernelCache {
    pub fn new() -> Self {
        KernelCache {
            memory: HashMap::new()
        }
    }
}

pub trait KernelRegister {
    fn register(&self, kernel: &Kernel) -> ModuleRef;
}

impl KernelRegister for Module {
    fn register(&self, kernel: &Kernel) -> ModuleRef {
        let mut imports = ImportsBuilder::new();
        imports.push_resolver("kenel", kernel);

        ModuleInstance::new(
            self,
            &imports,
        ).expect("Failed to instantiate module")
            .assert_no_start()
    }
}

pub struct Kernel {
    runtimes: Vec<Runtime>,
    final_result: Option<RuntimeResult>,
    cache: KernelCache
}

impl Kernel {
    pub fn new(addr: Address, input_balance: u64) -> Result<Self, Error> {
        let mut kernel = Kernel {
            runtimes: Vec::new(),
            final_result: None,
            cache: KernelCache::new()
        };

        let runtime_result = kernel.init_base_runtime(addr, input_balance);
        match runtime_result {
            Ok(runtime) => {
                kernel.push_runtime(runtime);
                Ok(kernel)
            },
            Err(e) => Err(e)
        }
    }

    pub fn run<'a>(&'a mut self, selector: Selector) -> &'a Option<RuntimeResult> {
        let result = self.execute_top_runtime(selector);
        self.merge_ret_result(result);
        self.final_result()
    }

    pub fn fork_runtime(
        &mut self,
        input_balance: u64,
        parent: &Runtime,
        selector: Selector,
        addr: Address) -> Result<(), Error> {
        self.init_runtime_with_parent(parent, addr, input_balance).and_then(|runtime| {
            match self.push_runtime(runtime) {
                Ok(size) => {
                    let ret =  self.execute_top_runtime(selector);
                    self.pop_runtime();
                    self.merge_ret_result(ret);
                    Ok(())
                },
                Err(e) => Err(e)
            }
        })
    }

    pub fn final_result<'a>(&'a self) -> &'a Option<RuntimeResult> {
        &self.final_result
    }

    pub fn top_runtime_mut<'a>(&'a mut self) -> &'a mut Runtime{
        self.runtimes.last_mut().unwrap()
    }

    pub fn cache(&self) -> KernelCache {
        self.cache.clone()
    }

    fn merge_ret_result(&mut self, ret: RuntimeResult) -> Result<(), Error> {
        unimplemented!()
    }

    fn init_base_runtime(&self, addr: Address, input_balance: u64) -> Result<Runtime, Error> {
        let mut code: Vec<u8> = vec![];
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code[..]).and_then(|_| {
                let init_runtime = Runtime::new(
                    account,
                    0usize,
                    self,
                    &code[..],
                    input_balance
                );
                Ok(init_runtime)
            })
        })
    }

    fn init_runtime_with_parent(&self, parent: &Runtime, addr: Address, input_balance: u64) -> Result<Runtime, Error> {
        if input_balance > parent.input_balance() {
            return Err(Error::Validation("Insufficient balance".into()));
        }
        let mut code: Vec<u8> = vec![];
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code[..]).and_then(|_| {
                let child_runtime = Runtime::new(
                    account,
                    parent.depth() + 1,
                    self,
                    &code[..],
                    input_balance
                );
                Ok(child_runtime)
            })
        })
    }

    fn push_runtime(&mut self, runtime: Runtime) -> Result<usize, Error> {
        unimplemented!()
    }

    fn pop_runtime(&mut self) -> Result<usize, Error> {
        unimplemented!()
    }

    fn execute_top_runtime(&mut self, selector: Selector) -> RuntimeResult {
        unimplemented!()
    }

    fn stack_depth(&self) -> usize {
        unimplemented!()
    }

    fn load_contract_account(account_addr: Address) -> Result<Account, Error> {
        unimplemented!()
    }

    fn load_code(account: &Account, code_buff: &mut [u8]) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Externals for Kernel {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            CALL_INDEX => void!(SystemCall::call()),
            _ => panic!("unknown function index {}", index)
        }
    }
}

impl ModuleImportResolver for Kernel {
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

impl Drop for Kernel {
    fn drop(&mut self) {

    }
}