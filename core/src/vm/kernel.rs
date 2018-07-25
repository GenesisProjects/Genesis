use account::Account;
use action::Action;
use common::address::Address;
use common::hash::Hash;

use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
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
    runtimes: Vec<Runtime>,
    final_result: Option<Result<RuntimeResult, Error>>,
    cache: KernelCache,
}

impl Kernel {
    pub fn new(addr: Address, input_balance: u64) -> Result<KernelRef, Error> {
        let mut kernel = Kernel {
            runtimes: Vec::new(),
            final_result: None,
            cache: KernelCache::new(),
        };

        let runtime_result = kernel.init_base_runtime(addr, input_balance);
        match runtime_result {
            Ok(runtime) => {
                kernel.push_runtime(runtime);
                Ok(Rc::new(RefCell::new(kernel)))
            }
            Err(e) => Err(e)
        }
    }

    pub fn run(&mut self, selector: Selector, system_call: &mut SystemCall) -> Result<RuntimeResult, Error> {
        self.execute_top_runtime(selector, system_call)
    }

    pub fn fork_runtime(
        &mut self,
        input_balance: u64,
        parent: &Runtime,
        selector: Selector,
        addr: Address,
        system_call: &mut SystemCall) -> Result<(), Error> {
        self.init_runtime_with_parent(parent, addr, input_balance).and_then(|runtime| {
            match self.push_runtime(runtime) {
                Ok(size) => {
                    let ret = self.execute_top_runtime(selector, system_call);
                    self.pop_runtime();
                    self.merge_ret_result(ret);
                    Ok(())
                }
                Err(e) => Err(e)
            }
        })
    }

    pub fn final_result(self) -> Result<RuntimeResult, Error> {
        self.final_result.unwrap()
    }

    pub fn top_runtime_mut<'a>(&'a mut self) -> &'a mut Runtime {
        self.runtimes.last_mut().unwrap()
    }

    pub fn cache(&self) -> KernelCache {
        self.cache.clone()
    }

    fn merge_ret_result(&mut self, ret: Result<RuntimeResult, Error>) -> Result<RuntimeResult, Error> {
        unimplemented!()
    }

    fn init_base_runtime(&self, addr: Address, input_balance: u64) -> Result<Runtime, Error> {
        let mut code: Vec<u8> = vec![];
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code).and_then(|_| {
                let init_runtime = Runtime::new(
                    account,
                    0usize,
                    &SysCallResolver::new(),
                    &code[..],
                    input_balance,
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
    }

    fn push_runtime(&mut self, runtime: Runtime) -> Result<usize, Error> {
        self.runtimes.push(runtime);
        Ok(self.runtimes.len())
    }

    fn pop_runtime(&mut self) -> Result<usize, Error> {
        match self.runtimes.pop() {
            Some(val) => Ok(self.runtimes.len()),
            None => Err(Error::Validation("no elements in runtimes".into()))
        }
    }

    fn execute_top_runtime(&mut self, selector: Selector, system_call: &mut SystemCall) -> Result<RuntimeResult, Error> {
        let runtime = self.top_runtime_mut();
        Self::execute(runtime, system_call, selector, 30usize)
    }

    pub fn execute(
        runtime: &mut Runtime,
        sys_call_ref: &mut SystemCall,
        selector: Selector,
        time_limit: usize
    ) -> Result<RuntimeResult, Error> {
        match runtime.module_ref() {
            Some(module_ref) => {
                match module_ref.invoke_export(
                    &selector.name()[..],
                    &selector.args(),
                    sys_call_ref
                ) {
                    Ok(ret) => {
                        Ok(RuntimeResult::new_with_ret(ret))
                    },
                    Err(e) => Err(e)
                }
            },
            None => {
                Err(Error::Validation("No module ref".into()))
            }
        }
    }

    fn stack_depth(&self) -> usize {
        self.runtimes.len()
    }

    fn load_contract_account(account_addr: Address) -> Result<Account, Error> {
        //unimplemented!()

        //TODO: test
        Ok(Account {})
    }

    fn load_code(account: &Account, code_buff: &mut Vec<u8>) -> Result<(), Error> {
        //unimplemented!()

        //TODO: test
        let mut f = File::open("./test_contract/test.wasm").expect("file not found");
        let mut contents = String::new();
        f.read_to_end(code_buff)
            .expect("something went wrong reading the file");
        Ok(())
    }
}