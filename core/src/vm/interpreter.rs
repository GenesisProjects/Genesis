extern crate common;
extern crate num;
extern crate rlp;
extern crate gen_utils;

use std::env::args;
use std::fs::File;

/// Function interpreter.
pub struct Interpreter {

}

/// Function execution context.
pub struct FunctionContext {
	/// Is context initialized.
	pub is_initialized: bool,
	/// Internal function reference.
	pub function: FuncRef,
	pub module: ModuleRef,
	/// Function return type.
	pub return_type: BlockType,
	/// Local variables.
	pub locals: Vec<RuntimeValue>,
	/// Values stack.
	pub value_stack: StackWithLimit<RuntimeValue>,
	/// Blocks frames stack.
	pub frame_stack: StackWithLimit<BlockFrame>,
	/// Current instruction position.
	pub position: usize,
}


/// Function run result.
enum RunResult {
	/// Function has returned (optional) value.
	Return(Option<RuntimeValue>),
	/// Function is calling other function.
	NestedCall(FuncRef),
}

impl<'a, E: Externals> Interpreter<'a, E> {
	pub fn start_execution(&mut self, ) -> Result< , > {


		self.run_interpreter_loop(&mut function_stack)
	}

	fn run_interpreter_loop(&mut self, ) -> Result< , > {
		loop {
			self.do_run_function(&mut function_context, function_body.opcodes.elements(), &function_body.labels)
		}
	}

	fn do_run_function(&mut self, ) -> Result< , > {
		//TBA
	}
}