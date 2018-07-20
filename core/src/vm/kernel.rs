use std::cell::RefCell;
use std::collections::LinkedList;

use wasmi::*;

use super::abi::Selector;
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

pub trait KernelRegister {
    fn regist(&self, kernel: &Kernel) -> ModuleRef;
}

impl KernelRegister for Module {
    fn regist(&self, kernel: &Kernel) -> ModuleRef {
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
    last_return_val: Option<RuntimeResult>,

    //account: Account,
    //tx: Transaction
    contract_info: Contract
}

impl Kernel {
    pub fn new() -> Self {
        unimplemented!();
    }

    fn commit(&mut self) -> Result<(), Error> {
        unimplemented!()
    }


    pub fn ext_call(&mut self, selector: Selector, account_addr: Address) -> Result<Runtime, Error> {
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
        self.commit();
    }
}