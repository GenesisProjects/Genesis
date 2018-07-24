extern crate common;
extern crate gen_core;
use common::address::Address;
use gen_core::vm::gen_vm::GenVM;
use gen_core::action::Action;
use gen_core::account::Account;

fn main() {
    let test_action = Action {};
    let test_addr = Address::load().unwrap();
    let mut vm = GenVM::new(&test_action, test_addr).unwrap();
    vm.init();
    loop {

    }
}