use account::Account;
use action::Action;
use common::address::Address;
use common::hash::Hash;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

    /*fn init_runtime_with_parent(&self, parent: &Runtime, addr: Address, input_balance: u64) -> Result<Runtime, Error> {
        if input_balance > parent.input_balance() {
            return Err(Error::Validation("Insufficient balance".into()));
        }
        let mut code: Vec<u8> = vec![];
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code).and_then(|_| {
                let child_runtime = Runtime::new(
                    account,
                    parent.depth() + 1,
                    &SysCallResolver::new(),
                    &code[..],
                    input_balance,
                );
                Ok(child_runtime)
            })
        })
    }*/
}