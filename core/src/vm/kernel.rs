use account::Account;
use action::Action;
use common::address::Address;
use common::hash::Hash;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use super::gen_vm::GenVM;
use super::runtime::*;
use super::selector::Selector;
use super::system_call::*;

use wasmi::*;

pub type CHUNK = [u8; 32];

pub type KernelRef = Rc<RefCell<Kernel>>;

#[derive(Clone)]
pub struct KernelCache {
    memory: HashMap<Hash, CHUNK>
}

impl KernelCache {
    pub fn new() -> Self {
        KernelCache {
            memory: HashMap::new()
        }
    }
}

pub struct Kernel {
    cache: KernelCache,
}

impl Kernel {
    pub fn new() -> KernelRef {
        Rc::new(RefCell::new(Kernel {
            cache: KernelCache::new(),
        }))
    }

    pub fn cache<'a>(&'a self) -> &'a KernelCache {
        &self.cache
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