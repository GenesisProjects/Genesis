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

		self.run_interpreter_loop(&mut function_stack)
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
					match function_stack.back_mut() {
						Some(caller_context) => if let Some(return_value) = return_value {
							caller_context.value_stack_mut().push(return_value).map_err(Trap::new)?;
						},
						None => return Ok(return_value),
					}
				},
				FuncInstanceInternal::Host { ref signature, .. } => {
					match *nested_func.as_internal() {
						FuncInstanceInternal::Internal { .. } => {
							let nested_context = function_context.nested(nested_func.clone()).map_err(Trap::new)?;
							function_stack.push_back(function_context);
							function_stack.push_back(nested_context);
						},
						FuncInstanceInternal::Host { ref signature, .. } => {
							let args = prepare_function_args(signature, &mut function_context.value_stack);
							let return_val = FuncInstance::invoke(&nested_func, &args, self.externals)?;

							// Check if `return_val` matches the signature.
							let value_ty = return_val.clone().map(|val| val.value_type());
							let expected_ty = nested_func.signature().return_type();
							if value_ty != expected_ty {
								return Err(TrapKind::UnexpectedSignature.into());
							}

							if let Some(return_val) = return_val {
								function_context.value_stack_mut().push(return_val).map_err(Trap::new)?;
							}
							function_stack.push_back(function_context);
						}
					}
				}
			}
		}
	}

	fn do_run_function(&mut self, ) -> Result< , > {
		loop {
			let instruction = &function_body[function_context.position];

			match self.run_instruction(function_context, function_labels, instruction)? {
				InstructionOutcome::RunNextInstruction => function_context.position += 1,
				InstructionOutcome::Branch(mut index) => {
					// discard index - 1 blocks
					while index >= 1 {
						function_context.discard_frame();
						index -= 1;
					}

					function_context.pop_frame(true)?;
					if function_context.frame_stack().is_empty() {
						break;
					}
				},
				InstructionOutcome::ExecuteCall(func_ref) => {
					function_context.position += 1;
					return Ok(RunResult::NestedCall(func_ref));
				},
				InstructionOutcome::End => {
					if function_context.frame_stack().is_empty() {
						break;
					}
				},
				InstructionOutcome::Return => break,
			}
		}

		Ok(RunResult::Return(match function_context.return_type {
			BlockType::Value(_) => {
				let result = function_context
					.value_stack_mut()
					.pop();
				Some(result)
			},
			BlockType::NoResult => None,
		}))
	}

}