use std::cell::RefCell;
use std::collections::{LinkedList, HashMap};

use wasmi::*;

use super::selector::Selector;
use super::gen_vm::GenVM;
use super::system_call::*;
use super::runtime::*;
use super::contract::Contract;
use common::address::Address;
use account::Account;

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
    runtimes: LinkedList<Runtime>,
    final_result: Option<RuntimeResult>,

    contract_info: Contract,
    cache: KernelCache
}

impl Kernel {
    pub fn new(init_runtime: Runtime) -> Self {
        unimplemented!();
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

    fn load_contract_account(&mut self, account_addr: Address) -> Result<Account, Error> {
        unimplemented!()
    }

    fn load_code(&mut self, account_ref: &Account, code_buff: &mut [u8]) -> Result<(), Error> {
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