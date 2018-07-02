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

struct Env {
	table_base: GlobalRef,
	memory_base: GlobalRef,
	memory: MemoryRef,
	table: TableRef,
}

impl Env {
	fn new() -> Env {
		Env {
			table_base: GlobalInstance::alloc(RuntimeValue::I32(0), false),
			memory_base: GlobalInstance::alloc(RuntimeValue::I32(0), false),
			memory: MemoryInstance::alloc(Pages(256), None).unwrap(),
			table: TableInstance::alloc(64, None).unwrap(),
		}
	}
}

impl ModuleImportResolver for Env {
	fn resolve_func(&self, _field_name: &str, _func_type: &Signature) -> Result<FuncRef, Error> {
		Err(Error::Instantiation(
			"env module doesn't provide any functions".into(),
		))
	}

	fn resolve_global(
		&self,
		field_name: &str,
		_global_type: &GlobalDescriptor,
	) -> Result<GlobalRef, Error> {
		match field_name {
			"tableBase" => Ok(self.table_base.clone()),
			"memoryBase" => Ok(self.memory_base.clone()),
			_ => Err(Error::Instantiation(format!(
				"env module doesn't provide global '{}'",
				field_name
			))),
		}
	}

	fn resolve_memory(
		&self,
		field_name: &str,
		_memory_type: &MemoryDescriptor,
	) -> Result<MemoryRef, Error> {
		match field_name {
			"memory" => Ok(self.memory.clone()),
			_ => Err(Error::Instantiation(format!(
				"env module doesn't provide memory '{}'",
				field_name
			))),
		}
	}

	fn resolve_table(&self, field_name: &str, _table_type: &TableDescriptor) -> Result<TableRef, Error> {
		match field_name {
			"table" => Ok(self.table.clone()),
			_ => Err(Error::Instantiation(
				format!("env module doesn't provide table '{}'", field_name),
			)),
		}
	}
}
