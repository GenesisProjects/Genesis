use chrono::*;
use wasmi::*;
use parity_wasm::elements::{self, Deserialize};

use super::selector::*;
use super::redo_log::Pipeline;
use super::kernel::*;

pub struct Runtime {
    depth: usize,
    module_ref: Option<ModuleRef>,
    input_balance: u64
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
        depth: usize,
        kernel_ref: &Kernel,
        buff: &[u8],
        input_balance: u64
    ) -> Self {
        let module = Module::from_buffer(buff).unwrap();
        Runtime {
            depth: depth,
            module_ref: Some(module.register(kernel_ref)),
            input_balance: input_balance
        }
    }

    pub fn new_with_local_contract(
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
                let ret = module_ref.invoke_export(
                    &selector.name()[..],
                    &selector.args(),
                    kenel_ref
                ).unwrap();
                unimplemented!()
            },
            None => {
                unimplemented!()
            }
        }
        unimplemented!()
    }
}

pub struct RuntimeResult {
    return_val: Option<Argument>,
    pipeline: Pipeline,
    success: bool,
    total_storage_alloc: usize,
    total_storage_free: usize,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    error: Option<Error>
}