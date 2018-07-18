use chrono::*;
use wasmi::*;

use super::abi::*;
use super::redo_log::Pipeline;

pub struct Runtime {
    module_instance: Option<ModuleInstance>
}

impl Runtime {
    pub fn new(buff: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn new_with_contract(name: &'static str) -> Self {
        unimplemented!()
    }

    pub fn execute(&mut self, abi: Function) -> RuntimeResult {
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

