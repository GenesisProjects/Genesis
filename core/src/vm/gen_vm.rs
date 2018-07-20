use super::abi::Selector;
use super::kernel::Kernel;
use super::runtime::*;

use wasmi::*;

use account::Account;
use transaction::Transaction;
use common::address::Address;

pub struct GenVM {
    kernel: Kernel,
}

impl GenVM {
    pub fn load_contract_account(&mut self, account_addr: Address) -> Result<Runtime, Error> {
        unimplemented!()
    }

    pub fn bootstrap(&mut self, tx: &Transaction) {
        unimplemented!()
    }
}