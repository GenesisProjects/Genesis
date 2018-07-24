use super::selector::Selector;
use super::kernel::Kernel;
use super::runtime::*;
use super::system_call::SystemCall;

use account::Account;
use action::Action;
use common::address::Address;

use wasmi::*;

pub struct GenVM<'a> {
    system_call: SystemCall<'a>,
    kernel: Kernel
}

impl <'a> GenVM<'a> {
    pub fn new(action: &Action, contract: Address) -> Result<Self, Error> {
        let input = GenVM::get_input_balance(action).unwrap();
        let mut system_call = SystemCall::new();
        Kernel::new(contract, input).and_then(|mut kernel| {
            let mut vm = GenVM {
                system_call: system_call,
                kernel: kernel
            };
            Ok(vm)
        })
    }

    pub fn init(&'a mut self) {
        self.system_call.init_with_kernel(&mut self.kernel);
    }

    pub fn launch<'b>(&'a mut self, action: &'b Action) -> &'a Option<RuntimeResult> {
        let selector: Selector = Selector::from(action.clone());
        self.kernel.run(selector, &mut self.system_call)
    }

    pub fn commit_result(&self, action: &mut Action, result: RuntimeResult) -> Result<(), Error> {
        unimplemented!()
    }

    fn get_input_balance(action: &Action) -> Result<u64, Error> {
        unimplemented!()
    }

}