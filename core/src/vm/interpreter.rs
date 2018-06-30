use std::mem;
use std::ops;
use std::{u32, usize};
use std::fmt;
use std::iter::repeat;
use std::collections::{HashMap, VecDeque};
use function::{FuncRef, FuncInstance, FuncInstanceInternal};
use parity_wasm::elements::{Opcode, BlockType, Local};
use {Error, Trap, TrapKind, Signature};
use common::{DEFAULT_MEMORY_INDEX, DEFAULT_TABLE_INDEX, BlockFrame, BlockFrameType};
use common::stack::StackWithLimit;

/// Maximum number of entries in value stack.
pub const DEFAULT_VALUE_STACK_LIMIT: usize = 16384;
/// Maximum number of entries in frame stack.
pub const DEFAULT_FRAME_STACK_LIMIT: usize = 16384;

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

impl FunctionContext {
	pub fn new(function: FuncRef, value_stack_limit: usize, frame_stack_limit: usize, signature: &Signature, args: Vec<RuntimeValue>) -> Self {
		let module = match *function.as_internal() {
			FuncInstanceInternal::Internal { ref module, .. } => module.upgrade().expect("module deallocated"),
			FuncInstanceInternal::Host { .. } => panic!("Host functions can't be called as internally defined functions; Thus FunctionContext can be created only with internally defined functions; qed"),
		};
		FunctionContext {
			is_initialized: false,
			function: function,
			module: ModuleRef(module),
			return_type: signature.return_type().map(|vt| BlockType::Value(vt.into_elements())).unwrap_or(BlockType::NoResult),
			value_stack: ValueStack::with_limit(value_stack_limit),
			frame_stack: StackWithLimit::with_limit(frame_stack_limit),
			locals: args,
			position: 0,
		}
	}

	pub fn nested(&mut self, function: FuncRef) -> Result<Self, TrapKind> {
		let module = match *function.as_internal() {
			FuncInstanceInternal::Internal { ref module, .. } => module.upgrade().expect("module deallocated"),
			FuncInstanceInternal::Host { .. } => panic!("host func cannot be executed"),
		};
		let func_type = function.signature();
		let func_return_type = func_type.return_type().map(|vt| BlockType::Value(vt.into_elements())).unwrap_or(BlockType::NoResult);
		let func_locals = prepare_func_args(func_type, &mut self.value_stack);

		Ok(FunctionContext {
			is_initialized: false,
			function: function,
			module: ModuleRef(module),
			return_type: func_return_type,
			value_stack: ValueStack::with_limit(self.value_stack.limit() - self.value_stack.len()),
			frame_stack: StackWithLimit::with_limit(self.frame_stack.limit() - self.frame_stack.len()),
			locals: func_locals,
			position: 0,
		})
	}

	pub fn initialize(&mut self, locals: &[Local]) {
		debug_assert!(!self.is_initialized);
		self.is_initialized = true;

		let locals = locals.iter()
			.flat_map(|l| repeat(l.value_type()).take(l.count() as usize))
			.map(::types::ValueType::from_elements)
			.map(RuntimeValue::default)
			.collect::<Vec<_>>();
		self.locals.extend(locals);
	}

	pub fn push_frame(&mut self, labels: &HashMap<usize, usize>, frame_type: BlockFrameType, block_type: BlockType) -> Result<(), TrapKind> {
		let begin_position = self.position;
		let branch_position = match frame_type {
			BlockFrameType::Function => usize::MAX,
			BlockFrameType::Loop => begin_position,
			BlockFrameType::IfTrue => {
				let else_pos = labels[&begin_position];
				1usize + match labels.get(&else_pos) {
					Some(end_pos) => *end_pos,
					None => else_pos,
				}
			},
			_ => labels[&begin_position] + 1,
		};
		let end_position = match frame_type {
			BlockFrameType::Function => usize::MAX,
			_ => labels[&begin_position] + 1,
		};

		self.frame_stack.push(BlockFrame {
			frame_type: frame_type,
			block_type: block_type,
			begin_position: begin_position,
			branch_position: branch_position,
			end_position: end_position,
			value_stack_len: self.value_stack.len(),
			polymorphic_stack: false,
		}).map_err(|_| TrapKind::StackOverflow)?;

		Ok(())
	}
}

/// Function run result.
enum RunResult {
	/// Function has returned (optional) value.
	Return(Option<RuntimeValue>),
	/// Function is calling other function.
	NestedCall(FuncRef),
}

impl<'a, E: Externals> Interpreter<'a, E> {
	pub fn start_execution(&mut self, func: &FuncRef, args: &[RuntimeValue]) -> Result<Option<RuntimeValue>, Trap> {
		let func_context = FunctionContext::new(
			func.clone(),
			DEFAULT_VALUE_STACK_LIMIT,
			DEFAULT_FRAME_STACK_LIMIT,
			func.signature(),
			args.into_iter().cloned().collect(),
		);

		let mut func_stack = VecDeque::new();
		func_stack.push_back(func_context);

		self.run_interpreter_loop(&mut func_stack)
	}

	fn run_interpreter_loop(&mut self, func_stack: &mut VecDeque<FunctionContext>) -> Result<Option<RuntimeValue>, Trap> {
		loop {
			let mut func_context = func_stack.pop_back().expect("no func context");
			let func_ref = func_context.function;
			let func_body = func_ref.body().expect("no body function");

			if !func_context.is_initialized() {
				let return_type = func_context.return_type;
				func_context.initialize(&func_body.locals);
				func_context.push_frame(&func_body.labels, BlockFrameType::Function, return_type).map_err(Trap::new)?;
			}

			let func_return = self.do_run_function(&mut func_context, func_body.instructions.elements(), &func_body.labels).map_err(Trap::new)?;
			
			match func_return {
				RunResult::Return(return_value) => {
					match func_stack.back_mut() {
						Some(caller_context) => if let Some(return_value) = return_value {
							caller_context.value_stack_mut().push(return_value).map_err(Trap::new)?;
						},
						None => return Ok(return_value),
					}
				},
				FuncInstanceInternal::Host { ref signature, .. } => {
					match *nested_func.as_internal() {
						FuncInstanceInternal::Internal { .. } => {
							let nested_context = func_context.nested(nested_func.clone()).map_err(Trap::new)?;
							func_stack.push_back(func_context);
							func_stack.push_back(nested_context);
						},
						FuncInstanceInternal::Host { ref signature, .. } => {
							let args = prepare_func_args(signature, &mut func_context.value_stack);
							let return_val = FuncInstance::invoke(&nested_func, &args, self.externals)?;

							// Check if `return_val` matches the signature.
							let value_ty = return_val.clone().map(|val| val.value_type());
							let expected_ty = nested_func.signature().return_type();
							if value_ty != expected_ty {
								return Err(TrapKind::UnexpectedSignature.into());
							}

							if let Some(return_val) = return_val {
								func_context.value_stack_mut().push(return_val).map_err(Trap::new)?;
							}
							func_stack.push_back(func_context);
						}
					}
				}
			}
		}
	}

	fn do_run_function(&mut self, ) -> Result< , > {
		loop {
			let instruction = &func_body[func_context.position];

			match self.run_instruction(func_context, func_labels, instruction)? {
				InstructionOutcome::RunNextInstruction => func_context.position += 1,
				InstructionOutcome::Branch(mut index) => {
					// discard index - 1 blocks
					while index >= 1 {
						func_context.discard_frame();
						index -= 1;
					}

					func_context.pop_frame(true)?;
					if func_context.frame_stack().is_empty() {
						break;
					}
				},
				InstructionOutcome::ExecuteCall(func_ref) => {
					func_context.position += 1;
					return Ok(RunResult::NestedCall(func_ref));
				},
				InstructionOutcome::End => {
					if func_context.frame_stack().is_empty() {
						break;
					}
				},
				InstructionOutcome::Return => break,
			}
		}

		Ok(RunResult::Return(match func_context.return_type {
			BlockType::Value(_) => {
				let result = func_context
					.value_stack_mut()
					.pop();
				Some(result)
			},
			BlockType::NoResult => None,
		}))
	}

}