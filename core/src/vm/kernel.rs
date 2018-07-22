use std::cell::RefCell;
use std::collections::{LinkedList, HashMap};

use wasmi::*;

use super::selector::Selector;
use super::gen_vm::GenVM;
use super::system_call::*;
use super::runtime::*;
use common::address::Address;
use common::hash::Hash;
use account::Account;
use action::Action;

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
    account: Option<Account>,
    runtimes: LinkedList<Runtime>,
    final_result: Option<RuntimeResult>,

    cache: KernelCache
}

impl Kernel {
    pub fn new(action: Action, addr: Address) -> Result<Self, Error> {
        let mut kernel = Kernel {
            account: None,
            runtimes: LinkedList::new(),
            final_result: None,

            cache: KernelCache::new()
        };
        let mut code: Vec<u8> = vec![];

        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code[..]).and_then(|_| {
                Kernel::get_input_balance(&action).and_then(|input_balance| {
                    let selector: Selector = Selector::from(action);
                    let init_runtime = Runtime::new(0usize, &kernal, &code[..], input_balance);
                    kernel.account = Some(account);

                    match kernel.push_runtime(init_runtime) {
                        Ok(_) => Ok(kernel),
                        Err(e) => Err(e)
                    }
                })
            })
        })
    }

    pub fn fork_runtime(
        &mut self,
        parent: &Runtime,
        selector: Selector,
        addr: Address) -> Result<(), Error> {
        unimplemented!()
    }

    fn merge_ret_result(&mut self, ret: &RuntimeResult) -> Result<(), Error> {
        unimplemented!()
    }

    fn push_runtime(&mut self, runtime: Runtime) -> Result<usize, Error> {
        unimplemented!()
    }

    fn pop_runtime(&mut self) -> Result<Runtime, Error> {
        unimplemented!()
    }

    fn excecute_top_runtime(&mut self, selector: Selector) -> RuntimeResult {
        unimplemented!()
    }

    fn stack_depth(&self) -> usize {
        unimplemented!()
    }

    fn load_contract_account(account_addr: Address) -> Result<Account, Error> {
        unimplemented!()
    }

    fn get_input_balance(action: &Action) -> Result<u64, Error> {
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