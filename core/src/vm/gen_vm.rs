use super::selector::Selector;
use super::kernel::Kernel;
use super::runtime::*;

use account::Account;
use action::Action;
use common::address::Address;

use wasmi::*;

pub struct GenVM {
    kernel: Kernel,
}

impl GenVM {
    pub fn new(action: &Action, contract: Address) -> Result<Self, Error> {
        let input = GenVM::get_input_balance(action).unwrap();
        match Kernel::new(contract, input) {
            Ok(kernel) => Ok(
                GenVM {
                    kernel: kernel
                }
            ),
            Err(e) => Err(e)
        }
    }

    pub fn launch<'a, 'b>(&'a mut self, action: &'b Action) -> &'a Option<RuntimeResult> {
        let selector: Selector = Selector::from(action.clone());
        self.kernel.run(selector)
    }

    pub fn commit_result(&self, action: &mut Action, result: RuntimeResult) -> Result<(), Error> {
        unimplemented!()
    }

    fn get_input_balance(action: &Action) -> Result<u64, Error> {
        unimplemented!()
    }

}