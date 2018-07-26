use chrono::*;
use wasmi::*;
use wasmi::ModuleInstance;
use parity_wasm::elements::{self, Deserialize};

use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

use super::selector::*;
use super::system_call::*;

use account::Account;
use transaction::Transaction;

#[derive(Clone)]
pub struct RuntimeContext {
    account: Account,
    depth: usize,
    balance: u64
}

impl RuntimeContext {
    pub fn new(
        account: Account,
        depth: usize,
        input_balance: u64
    ) -> Self {
        RuntimeContext {
            account: account,
            depth: depth,
            balance: input_balance
        }
    }
}

pub struct Runtime {
    context: RuntimeContext,
    module_ref: Option<ModuleRef>
}

impl Runtime {
    /// # new(&mut self)
    /// **Usage**
    /// - Initiate Runtime with a wasm module instance
    /// **Parameters**
    /// - 1. ***&[u8](buff)***: the read buffer
    /// ## Examples
    /// ```
    /// ```
    pub fn new(
        account: Account,
        depth: usize,
        sys_resolver: &SysCallResolver,
        buff: &[u8],
        input_balance: u64
    ) -> Self {
        let module = Module::from_buffer(buff).unwrap();

        Runtime {
            context: RuntimeContext::new(account, depth, input_balance),
            module_ref: Some(module.register(sys_resolver)),
        }
    }

    pub fn new_with_local_contract(
        account: Account,
        depth: usize,
        sys_resolver: &SysCallResolver,
        path: &'static str,
        input_balance: usize
    ) -> Self {
       unimplemented!()
    }

    pub fn context(&self) -> RuntimeContext {
        self.context.clone()
    }

    pub fn depth(&self) -> usize {
        self.context.depth
    }

    pub fn input_balance(&self) -> u64 {
        self.context.balance
    }

    pub fn account_ref<'a>(&'a self) -> &'a Account {
        &self.context.account
    }

    pub fn module_ref(&self) -> Option<ModuleRef> {
        self.module_ref.clone()
    }

    pub fn memory_ref(&self) -> Option<MemoryRef> {
        let module_ref = self.module_ref().unwrap();
        module_ref.export_by_name("memory").and_then(|ext| {
            match ext {
                ExternVal::Memory(mem_ref) => Some(mem_ref),
                _ => None
            }
        })
    }
}

pub struct RuntimeResult {
    return_val: Option<Argument>,
    success: bool,
    total_storage_alloc: usize,
    total_storage_free: usize,
    txs: Vec<Transaction>
}

impl RuntimeResult {
    pub fn new_with_ret(ret: Option<RuntimeValue>) -> Self {
        unimplemented!()
    }
}