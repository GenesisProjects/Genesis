use std::cell::RefCell;
use std::collections::HashMap;

use wasmi::*;

use super::selector::Selector;
use super::gen_vm::GenVM;
use super::system_call::*;
use super::runtime::*;
use common::address::Address;
use common::hash::Hash;
use action::Action;
use account::Account;

pub type CHUNK = [u8; 32];

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
    runtimes: Vec<Runtime>,
    final_result: Option<RuntimeResult>,
    cache: KernelCache
}

impl Kernel {
    pub fn new(addr: Address, input_balance: u64) -> Result<Self, Error> {
        let mut kernel = Kernel {
            runtimes: Vec::new(),
            final_result: None,
            cache: KernelCache::new()
        };

        let runtime_result = kernel.init_base_runtime(addr, input_balance);
        match runtime_result {
            Ok(runtime) => {
                kernel.push_runtime(runtime);
                Ok(kernel)
            },
            Err(e) => Err(e)
        }
    }

    pub fn run<'a>(&'a mut self, selector: Selector) -> &'a Option<RuntimeResult> {
        let result = self.execute_top_runtime(selector);
        self.merge_ret_result(result);
        self.final_result()
    }

    pub fn fork_runtime(
        &mut self,
        input_balance: u64,
        parent: &Runtime,
        selector: Selector,
        addr: Address) -> Result<(), Error> {
        self.init_runtime_with_parent(parent, addr, input_balance).and_then(|runtime| {
            match self.push_runtime(runtime) {
                Ok(size) => {
                    let ret =  self.execute_top_runtime(selector);
                    self.pop_runtime();
                    self.merge_ret_result(ret);
                    Ok(())
                },
                Err(e) => Err(e)
            }
        })
    }

    pub fn final_result<'a>(&'a self) -> &'a Option<RuntimeResult> {
        &self.final_result
    }

    pub fn top_runtime_mut<'a>(&'a mut self) -> &'a mut Runtime{
        self.runtimes.last_mut().unwrap()
    }

    pub fn cache(&self) -> KernelCache {
        self.cache.clone()
    }

    fn merge_ret_result(&mut self, ret: RuntimeResult) -> Result<(), Error> {
        unimplemented!()
    }

    fn init_base_runtime(&self, addr: Address, input_balance: u64) -> Result<Runtime, Error> {
        let mut code: Vec<u8> = vec![];
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code[..]).and_then(|_| {
                let init_runtime = Runtime::new(
                    account,
                    0usize,
                    &SYSTEM_CALL.lock().unwrap(),
                    &code[..],
                    input_balance
                );
                Ok(init_runtime)
            })
        })
    }

    fn init_runtime_with_parent(&self, parent: &Runtime, addr: Address, input_balance: u64) -> Result<Runtime, Error> {
        if input_balance > parent.input_balance() {
            return Err(Error::Validation("Insufficient balance".into()));
        }
        let mut code: Vec<u8> = vec![];
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code[..]).and_then(|_| {
                let child_runtime = Runtime::new(
                    account,
                    parent.depth() + 1,
                    &SYSTEM_CALL.lock().unwrap(),
                    &code[..],
                    input_balance
                );
                Ok(child_runtime)
            })
        })
    }

    fn push_runtime(&mut self, runtime: Runtime) -> Result<usize, Error> {
        unimplemented!()
    }

    fn pop_runtime(&mut self) -> Result<usize, Error> {
        unimplemented!()
    }

    fn execute_top_runtime(&mut self, selector: Selector) -> RuntimeResult {
        unimplemented!()
    }

    fn stack_depth(&self) -> usize {
        unimplemented!()
    }

    fn load_contract_account(account_addr: Address) -> Result<Account, Error> {
        unimplemented!()
    }

    fn load_code(account: &Account, code_buff: &mut [u8]) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {

    }
}