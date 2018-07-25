use chrono::*;
use wasmi::*;
use parity_wasm::elements::{self, Deserialize};

use super::selector::*;
use super::system_call::*;

use account::Account;
use transaction::Transaction;

pub struct Runtime {
    account: Account,
    depth: usize,
    module_ref: Option<ModuleRef>,
    balance: u64
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
            account: account,
            depth: depth,
            module_ref: Some(module.register(sys_resolver)),
            balance: input_balance
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

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn input_balance(&self) -> u64 {
        self.balance
    }

    pub fn account_ref<'a>(&'a self) -> &'a Account {
        &self.account
    }

    pub fn module_ref(&self) -> Option<ModuleRef> {
        self.module_ref.clone()
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