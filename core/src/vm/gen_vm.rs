use super::selector::Selector;
use super::kernel::{Kernel, KernelRef};
use super::runtime::*;
use super::system_call::{SystemCall, SysCallResolver};

use account::Account;
use action::Action;
use common::address::Address;
use wasmi::*;

use std::fs::File;
use std::io::prelude::*;

pub struct GenVM{
    system_call: SystemCall,
    kernel: KernelRef
}

impl GenVM {
    pub fn new(action: &Action, contract: Address) -> Result<Self, Error> {
        let kernel_ref = Kernel::new();
        let mut vm = GenVM {
            system_call: SystemCall::new_with_kernel(kernel_ref.clone()),
            kernel: kernel_ref.clone()
        };
        Ok(vm)
    }

    pub fn launch(&mut self, action: &Action) -> Result<RuntimeResult, Error> {
        let selector: Selector = Selector::from(action.clone());
        self.init_base_runtime(
            action.addr.clone(),
            action.balance
        )
            .and_then(|mut runtime| {
            self.execute(&mut runtime, selector, 1000)
        })
    }

    pub fn commit_result(&self, action: &mut Action, result: RuntimeResult) -> Result<(), Error> {
        unimplemented!()
    }

    fn execute(
        &mut self,
        runtime: &mut Runtime,
        selector: Selector,
        time_limit: usize
    ) -> Result<RuntimeResult, Error> {
        match runtime.module_ref() {
            Some(module_ref) => {
                match module_ref.invoke_export(
                    &selector.name()[..],
                    &selector.args(),
                    &mut self.system_call
                ) {
                    Ok(ret) => {
                        Ok(RuntimeResult::new_with_ret(ret))
                    },
                    Err(e) => Err(e)
                }
            },
            None => {
                Err(Error::Validation("No module ref".into()))
            }
        }
    }

    fn init_base_runtime(&self, addr: Address, input_balance: u64) -> Result<Runtime, Error> {
        let mut code: Vec<u8> = vec![];
        GenVM::load_contract_account(addr).and_then(|account| {
            GenVM::load_code(&account, &mut code).and_then(|_| {
                let init_runtime = Runtime::new(
                    account,
                    0usize,
                    &SysCallResolver::new(),
                    &code[..],
                    input_balance,
                );
                Ok(init_runtime)
            })
        })
    }

    pub fn load_contract_account(account_addr: Address) -> Result<Account, Error> {
        //unimplemented!()

        //TODO: test
        Ok(Account {})
    }

    pub fn load_code(account: &Account, code_buff: &mut Vec<u8>) -> Result<(), Error> {
        //unimplemented!()

        //TODO: test
        let mut f = File::open("./test_contract/test.wasm").expect("file not found");
        let mut contents = String::new();
        f.read_to_end(code_buff)
            .expect("something went wrong reading the file");
        Ok(())
    }

}