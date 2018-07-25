use super::selector::Selector;
use super::kernel::{Kernel, KernelRef};
use super::runtime::*;
use super::system_call::{SystemCall, SysCallResolver};

use account::Account;
use action::Action;
use common::address::Address;
use wasmi::*;

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
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code).and_then(|_| {
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
}