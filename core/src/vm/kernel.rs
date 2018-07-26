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

pub const MAX_STACK_SIZE: usize = 16usize;

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
    stack: Vec<(RuntimeContextRef, MemoryRef, ModuleRef)>,
    result: Option<Result<RuntimeResult, Error>>
}

impl Kernel {
    pub fn new() -> KernelRef {
        Rc::new(RefCell::new(Kernel {
            cache: KernelCache::new(),
            stack: vec![],
            result: None
        }))
    }

    pub fn push_runtime(&mut self, context: RuntimeContextRef, memory: MemoryRef, module: ModuleRef) -> bool {
        if self.stack.len() > MAX_STACK_SIZE {
            false
        } else {
            self.stack.push((context, memory, module));
            true
        }
    }

    pub fn pop_runtime(&mut self) {
        self.stack.pop();
    }

    pub fn top_context(&self) -> RuntimeContextRef {
        self.stack.last().unwrap().0.clone()
    }

    pub fn top_memory(&self) -> MemoryRef {
        self.stack.last().unwrap().1.clone()
    }

    pub fn cache<'a>(&'a self) -> &'a KernelCache {
        &self.cache
    }

    pub fn load_contract_account(account_addr: Address) -> Result<Account, Error> {
        //unimplemented!()

        //TODO: test
        Ok(Account {
            balance: 0u32
        })
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

    pub fn merge_result(&mut self, new_result: &Result<RuntimeResult, Error>) {
        unimplemented!()
    }

}