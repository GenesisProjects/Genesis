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
    pub fn new_with_action(action: Action, contract: Address) -> Result<Self, Error> {
        match Kernel::new(action, contract) {
            Ok(kernel) => Ok(
                GenVM {
                    kernel: kernel
                }
            ),
            Err(e) => Err(e)
        }
    }

    pub fn launch(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn commit_result(&self, action: &mut Action) -> Result<(), Error> {
        unimplemented!()
    }
}