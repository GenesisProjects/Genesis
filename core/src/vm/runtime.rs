use chrono::*;
use wasmi::*;
use parity_wasm::elements::{self, Deserialize};

use super::abi::*;
use super::redo_log::Pipeline;
use super::kernel::*;

pub struct Runtime {
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
    pub fn new(buff: &[u8]) -> Self {
        let module = Module::from_buffer(buff).unwrap();
        let mut imports = ImportsBuilder::new();
        imports.push_resolver("env", &Kernel::bootstrap());

        let module_ref = ModuleInstance::new(
            &module,
            &imports,
        ).expect("Failed to instantiate module")
            .assert_no_start();

        Runtime {
            module_ref: Some(module_ref)
        }
    }

    pub fn new_with_contract(name: &'static str) -> Self {
        unimplemented!()
    }

    pub fn execute(&mut self, abi: Selector) -> RuntimeResult {
        unimplemented!()
    }

    pub fn commit(&mut self, pipeline: &Pipeline) -> Result<(), Error> {
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