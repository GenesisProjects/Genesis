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

pub type RuntimeContextRef = Rc<RefCell<RuntimeContext>>;

#[derive(Clone)]
pub struct RuntimeContext {
    pub account: Account,
    pub depth: usize,
    pub balance: u32
}

impl RuntimeContext {
    pub fn new(
        account: Account,
        depth: usize,
        input_balance: u32
    ) -> RuntimeContextRef {
        Rc::new(RefCell::new(RuntimeContext {
            account: account,
            depth: depth,
            balance: input_balance
        }))
    }
}

pub struct Runtime {
    context: RuntimeContextRef,
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
        input_balance: u32
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

    pub fn context(&self) -> RuntimeContextRef {
        self.context.clone()
    }

    pub fn depth(&self) -> usize {
        self.context.borrow().depth
    }

    pub fn input_balance(&self) -> u32 {
        self.context.borrow().balance
    }

    pub fn account(&self) -> Account {
        let context = self.context.borrow();
        context.account.clone()
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
    return_val: Option<RuntimeValue>,
    success: bool,
    txs: Vec<Transaction>
}

impl RuntimeResult {
    pub fn new_with_ret(ret: Option<RuntimeValue>) -> Self {
        unimplemented!()
    }

    pub fn return_val(&self) -> Option<RuntimeValue> {
        self.return_val.clone()
    }
}