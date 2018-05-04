extern crate common;
extern crate num;
extern crate rlp;
extern crate gen_utils;

use std::env::args;
use std::fs::File;

#[derive(Clone, Debug)]
pub struct ModuleRef(pub(crate) Rc<ModuleInstance>);

impl ::std::ops::Deref for ModuleRef {
	type Target = ModuleInstance;
	fn deref(&self) -> &ModuleInstance {
		&self.0
	}
}

/// An external value is the runtime representation of an entity
/// that can be imported or exported.
pub enum ExternVal {
	/// [Function][`FuncInstance`].
	///
	/// [`FuncInstance`]: struct.FuncInstance.html
	Func(FuncRef),
	/// [Table][`TableInstance`].
	///
	/// [`TableInstance`]: struct.TableInstance.html
	Table(TableRef),
	/// [Memory][`MemoryInstance`].
	///
	/// [`MemoryInstance`]: struct.MemoryInstance.html
	Memory(MemoryRef),
	/// [Global][`GlobalInstance`].
	///
	/// Should be immutable.
	///
	/// [`GlobalInstance`]: struct.GlobalInstance.html
	Global(GlobalRef),
}

impl Clone for ExternVal {
	fn clone(&self) -> Self {
		match *self {
			ExternVal::Func(ref func) => ExternVal::Func(func.clone()),
			ExternVal::Table(ref table) => ExternVal::Table(table.clone()),
			ExternVal::Memory(ref memory) => ExternVal::Memory(memory.clone()),
			ExternVal::Global(ref global) => ExternVal::Global(global.clone()),
		}
	}
}

impl fmt::Debug for ExternVal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"ExternVal {{ {} }}",
			match *self {
				ExternVal::Func(_) => "Func",
				ExternVal::Table(_) => "Table",
				ExternVal::Memory(_) => "Memory",
				ExternVal::Global(_) => "Global",
			}
		)
	}
}

impl ExternVal {
	/// Get underlying function reference if this `ExternVal` contains
	/// a function, or `None` if it is some other kind.
	pub fn as_func(&self) -> Option<&FuncRef> {
		match *self {
			ExternVal::Func(ref func) => Some(func),
			_ => None,
		}
	}

	/// Get underlying table reference if this `ExternVal` contains
	/// a table, or `None` if it is some other kind.
	pub fn as_table(&self) -> Option<&TableRef> {
		match *self {
			ExternVal::Table(ref table) => Some(table),
			_ => None,
		}
	}

	/// Get underlying memory reference if this `ExternVal` contains
	/// a memory, or `None` if it is some other kind.
	pub fn as_memory(&self) -> Option<&MemoryRef> {
		match *self {
			ExternVal::Memory(ref memory) => Some(memory),
			_ => None,
		}
	}

	/// Get underlying global variable reference if this `ExternVal` contains
	/// a global, or `None` if it is some other kind.
	pub fn as_global(&self) -> Option<&GlobalRef> {
		match *self {
			ExternVal::Global(ref global) => Some(global),
			_ => None,
		}
	}
}

#[derive(Debug)]
pub struct ModuleInstance {
	signatures: RefCell<Vec<Rc<Signature>>>,
	tables: RefCell<Vec<TableRef>>,
	funcs: RefCell<Vec<FuncRef>>,
	memories: RefCell<Vec<MemoryRef>>,
	globals: RefCell<Vec<GlobalRef>>,
	exports: RefCell<HashMap<String, ExternVal>>,
}

impl ModuleInstance {
	fn default() -> Self {
		ModuleInstance {
			funcs: RefCell::new(Vec::new()),
			signatures: RefCell::new(Vec::new()),
			tables: RefCell::new(Vec::new()),
			memories: RefCell::new(Vec::new()),
			globals: RefCell::new(Vec::new()),
			exports: RefCell::new(HashMap::new()),
		}
	}

	/// Instantiate a [module][`Module`].
	pub fn new<'m, I: ImportResolver>(
		loaded_module: &'m Module,
		imports: &I,
	) -> Result<NotStartedModuleRef<'m>, Error> {
		let module = loaded_module.module();

		let mut extern_vals = Vec::new();
		for import_entry in module.import_section().map(|s| s.entries()).unwrap_or(&[]) {
			let module_name = import_entry.module();
			let field_name = import_entry.field();
			let extern_val = match *import_entry.external() {
				External::Function(fn_ty_idx) => {
					let types = module.type_section().map(|s| s.types()).unwrap_or(&[]);
					let &Type::Function(ref func_type) = types
						.get(fn_ty_idx as usize)
						.expect("Due to validation functions should have valid types");
					let signature = Signature::from_elements(func_type);
					let func = imports.resolve_func(module_name, field_name, &signature)?;
					ExternVal::Func(func)
				}
				External::Table(ref table_type) => {
					let table_descriptor = TableDescriptor::from_elements(table_type);
					let table = imports.resolve_table(module_name, field_name, &table_descriptor)?;
					ExternVal::Table(table)
				}
				External::Memory(ref memory_type) => {
					let memory_descriptor = MemoryDescriptor::from_elements(memory_type);
					let memory = imports.resolve_memory(module_name, field_name, &memory_descriptor)?;
					ExternVal::Memory(memory)
				}
				External::Global(ref global_type) => {
					let global_descriptor = GlobalDescriptor::from_elements(global_type);
					let global = imports.resolve_global(module_name, field_name, &global_descriptor)?;
					ExternVal::Global(global)
				}
			};
			extern_vals.push(extern_val);
		}

		Self::with_externvals(loaded_module, extern_vals.iter())
	}


	/// Find export by a name.
	///
	/// Returns `None` if there is no export with such name.
	pub fn export_by_name(&self, name: &str) -> Option<ExternVal> {
		self.exports.borrow().get(name).cloned()
	}
}

