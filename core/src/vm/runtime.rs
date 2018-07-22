use chrono::*;
use wasmi::*;
use parity_wasm::elements::{self, Deserialize};

use super::selector::*;
use super::kernel::*;

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
        kernel_ref: &Kernel,
        buff: &[u8],
        input_balance: u64
    ) -> Self {
        let module = Module::from_buffer(buff).unwrap();
        Runtime {
            account: account,
            depth: depth,
            module_ref: Some(module.register(kernel_ref)),
            balance: input_balance
        }
    }

    pub fn new_with_local_contract(
        account: Account,
        depth: usize,
        kernel_ref: &Kernel,
        path: &'static str,
        input_balance: usize
    ) -> Self {
       unimplemented!()
    }

    pub fn execute(
        &mut self,
        kenel_ref: &mut Kernel,
        selector: Selector,
        time_limit: usize
    ) -> RuntimeResult {
        match self.module_ref {
            Some(ref module_ref) => {
                match module_ref.invoke_export(
                    &selector.name()[..],
                    &selector.args(),
                    kenel_ref
                ) {
                    Ok(ret) => {
                        RuntimeResult::new_with_ret(ret)
                    },
                    Err(e) => {
                        RuntimeResult::new_with_err(e)
                    }
                }

            },
            None => {
                RuntimeResult::new_with_err(Error::Validation("ModuleRef is none".into()))
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn input_balance(&self) -> u64 {
        self.balance
    }

    pub fn acount_ref<'a>(&'a self) -> &'a Account {
        &self.account
    }
}

pub struct RuntimeResult {
    return_val: Option<Argument>,
    success: bool,
    total_storage_alloc: usize,
    total_storage_free: usize,
    error: Option<Error>,
    txs: Vec<Transaction>
}

impl RuntimeResult {
    pub fn new_with_ret(ret: Option<RuntimeValue>) -> Self {
        unimplemented!()
    }

    pub fn new_with_err(err: Error) -> Self {
        unimplemented!()
    }
}