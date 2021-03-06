use account::Account;
use storage::*;

use common::address::Address;

use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;

use super::runtime::*;

use wasmi::*;

pub const MAX_STACK_SIZE: usize = 16usize;

pub type KernelRef = Rc<RefCell<Kernel>>;

pub struct Kernel {
    stack: Vec<(RuntimeContextRef, Option<MemoryRef>, ModuleRef, StorageCache)>,
    address: Address,
    result: Option<Result<RuntimeResult, Error>>
}

impl Kernel {
    pub fn new(address: Address) -> KernelRef {
        Rc::new(RefCell::new(Kernel {
            stack: vec![],
            address: address,
            result: None
        }))
    }

    #[inline]
    pub fn push_runtime(
        &mut self,
        context: RuntimeContextRef,
        memory: Option<MemoryRef>,
        module: ModuleRef,
        cache: StorageCache
    ) -> bool {
        if self.stack.len() > MAX_STACK_SIZE {
            false
        } else {
            self.stack.push(
                (
                    context,
                    memory,
                    module,
                    cache
                )
            );
            true
        }
    }

    #[inline]
    pub fn pop_runtime(&mut self) {
        self.stack.pop();
    }

    #[inline]
    pub fn top_context(&self) -> RuntimeContextRef {
        self.stack.last().unwrap().0.clone()
    }

    #[inline]
    pub fn top_memory(&self) -> Option<MemoryRef> {
        self.stack.last().unwrap().1.clone()
    }

    #[inline]
    pub fn top_cache_mut<'a>(&'a mut self) -> &'a mut StorageCache {
        &mut self.stack.last_mut().unwrap().3
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    #[inline]
    pub fn load_contract_account(account_addr: Address) -> Result<Account, Error> {
        Account::load(account_addr).map_err(|e| {
            Error::Validation("Can not load account".into())
        })
    }

    #[inline]
    pub fn load_code(account: &Account, code_buff: &mut Vec<u8>) -> Result<(), Error> {
        //unimplemented!()
        //TODO: test
        let mut f = File::open("./test_contract/test_loop.wasm").expect("file not found");
        let contents = String::new();
        f.read_to_end(code_buff)
            .expect("something went wrong reading the file");
        Ok(())
    }

    #[inline]
    pub fn merge_result(&mut self, new_result: &Result<RuntimeResult, Error>) {
        //unimplemented!()
    }

}
