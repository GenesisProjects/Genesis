use std::mem;
use std::ops;
use std::{u32, usize};
use std::fmt;
use std::iter::repeat;
use std::collections::{HashMap, VecDeque};
use func::{FuncRef, FuncInstance, FuncInstanceInternal};
use memory::{MemoryInstance, MemoryRef};
use global::{GlobalInstance, GlobalRef};
use module::{ModuleImportResolver, ModuleInstance}
use value::{
	RuntimeValue, FromRuntimeValue, WrapInto, TryTruncateInto, ExtendInto,
	ArithmeticOps, Integer, Float, LittleEndianConvert, TransmuteInto,
};
use parity_wasm::elements::{Opcode, BlockType, Local, Module};
use {Error, Trap, TrapKind, Signature};
use common::{DEFAULT_MEMORY_INDEX, DEFAULT_TABLE_INDEX, BlockFrame, BlockFrameType};
use common::stack::StackWithLimit;

use memory_units::Pages;
use std::fs::File;

/// Wrapped interpreter error
#[derive(Debug)]
pub enum Error {
	Interpreter(InterpreterError),
	Trap(Trap),
}

impl From<InterpreterError> for Error {
	fn from(e: InterpreterError) -> Self {
		Error::Interpreter(e)
	}
}

impl From<Trap> for Error {
	fn from(e: Trap) -> Self {
		Error::Trap(e)
	}
}

impl From<Error> for vm::Error {
	fn from(e: Error) -> Self {
		match e {
			Error::Interpreter(e) => vm::Error::Wasm(format!("Wasm runtime error: {:?}", e)),
			Error::Trap(e) => vm::Error::Wasm(format!("Wasm contract trap: {:?}", e)),
		}
	}
}

/// Wasm interpreter instance
pub struct WasmInterpreter;

impl From<runtime::Error> for vm::Error {
	fn from(e: runtime::Error) -> Self {
		vm::Error::Wasm(format!("Wasm runtime error: {:?}", e))
	}
}

enum ExecutionOutcome {
	Suicide,
	Return,
	NotSpecial,
}

impl WasmInterpreter {

	pub fn exec(&mut self, params: ActionParams, ext: &mut vm::Ext) -> vm::Result<GasLeft> {
		let (module, data) = parser::payload(&params, ext.schedule().wasm())?;

		let loaded_module = Module::from_parity_wasm_module(module).map_err(Error::Interpreter)?;

		let instantiation_resolver = env::ImportResolver::with_limit(16);

		let module_instance = wasmi::ModuleInstance::new(
			&loaded_module,
			&wasmi::ImportsBuilder::new().with_resolver("env", &instantiation_resolver)
		).map_err(Error::Interpreter)?;

		let initial_memory = instantiation_resolver.memory_size().map_err(Error::Interpreter)?;
		trace!(target: "wasm", "Contract requested {:?} pages of initial memory", initial_memory);

		let result = {
			let mut runtime = Runtime::with_params(
				ext,
				instantiation_resolver.memory_ref(),
				// cannot overflow, checked above
				data.to_vec(),
				RuntimeContext {
					address: params.address,
					sender: params.sender,
					origin: params.origin,
					code_address: params.code_address,
					value: params.value.value(),
				},
			);

			assert!(runtime.schedule().wasm().initial_mem < 1 << 16);
			runtime.charge(|s| initial_memory as u64 * s.wasm().initial_mem as u64)?;

			let module_instance = module_instance.run_start(&mut runtime).map_err(Error::Trap)?;

			let invoke_result = module_instance.invoke_export("call", &[], &mut runtime);

			let mut execution_outcome = ExecutionOutcome::NotSpecial;
			if let Err(InterpreterError::Trap(ref trap)) = invoke_result {
				if let wasmi::TrapKind::Host(ref boxed) = *trap.kind() {
					let ref runtime_err = boxed.downcast_ref::<runtime::Error>()
						.expect("Host errors other than runtime::Error never produced; qed");

					match **runtime_err {
						runtime::Error::Suicide => { execution_outcome = ExecutionOutcome::Suicide; },
						runtime::Error::Return => { execution_outcome = ExecutionOutcome::Return; },
						_ => {}
					}
				}
			}

			if let (ExecutionOutcome::NotSpecial, Err(e)) = (execution_outcome, invoke_result) {
				trace!(target: "wasm", "Error executing contract: {:?}", e);
				return Err(vm::Error::from(Error::from(e)));
			}

			(
				runtime.gas_left().expect("Cannot fail since it was not updated since last charge"),
				runtime.into_result(),
			)
		};


		if result.is_empty() {
			trace!(target: "wasm", "Contract execution result is empty.");
			Ok()
		} else {
			let len = result.len();
			Ok({
				data: ReturnData::new(
					result,
					0,
					len,
				),
				apply_state: true,
			})
		}
	}
}