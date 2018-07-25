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

}