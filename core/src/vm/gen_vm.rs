use super::selector::Selector;
use super::kernel::{Kernel, KernelRef};
use super::runtime::*;
use super::system_call::SystemCall;

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
        let input = GenVM::get_input_balance(action).unwrap();
        Kernel::new(contract, input).and_then(|mut kernel| {
            let mut vm = GenVM {
                system_call: SystemCall::new_with_kernel(kernel.clone()),
                kernel: kernel
            };
            Ok(vm)
        })
    }

    pub fn launch<'a>(&mut self, action: &'a Action) -> Result<RuntimeResult, Error> {
        let selector: Selector = Selector::from(action.clone());
        self.kernel.borrow_mut().run(selector, &mut self.system_call)
    }

    pub fn commit_result(&self, action: &mut Action, result: RuntimeResult) -> Result<(), Error> {
        unimplemented!()
    }

    fn get_input_balance(action: &Action) -> Result<u64, Error> {
        //unimplemented!()
        //TODO: test
        Ok(100)
    }

}