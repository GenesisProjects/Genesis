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

	fn run_instruction(&mut self, context: &mut FunctionContext, labels: &HashMap<usize, usize>, instruction: &Instruction) -> Result<InstructionOutcome, TrapKind> {
		match instruction {
			&Instruction::Unreachable => self.run_unreachable(context),
			&Instruction::Nop => self.run_nop(context),
			&Instruction::Block(block_type) => self.run_block(context, labels, block_type),
			&Instruction::Loop(block_type) => self.run_loop(context, labels, block_type),
			&Instruction::If(block_type) => self.run_if(context, labels, block_type),
			&Instruction::Else => self.run_else(context, labels),
			&Instruction::End => self.run_end(context),
			&Instruction::Br(idx) => self.run_br(context, idx),
			&Instruction::BrIf(idx) => self.run_br_if(context, idx),
			&Instruction::BrTable(ref table, default) => self.run_br_table(context, table, default),
			&Instruction::Return => self.run_return(context),

			&Instruction::Call(index) => self.run_call(context, index),
			&Instruction::CallIndirect(index, _reserved) => self.run_call_indirect(context, index),

			&Instruction::Drop => self.run_drop(context),
			&Instruction::Select => self.run_select(context),

			&Instruction::GetLocal(index) => self.run_get_local(context, index),
			&Instruction::SetLocal(index) => self.run_set_local(context, index),
			&Instruction::TeeLocal(index) => self.run_tee_local(context, index),
			&Instruction::GetGlobal(index) => self.run_get_global(context, index),
			&Instruction::SetGlobal(index) => self.run_set_global(context, index),

			&Instruction::I32Load(align, offset) => self.run_load::<i32>(context, align, offset),
			&Instruction::I64Load(align, offset) => self.run_load::<i64>(context, align, offset),
			&Instruction::F32Load(align, offset) => self.run_load::<F32>(context, align, offset),
			&Instruction::F64Load(align, offset) => self.run_load::<F64>(context, align, offset),
			&Instruction::I32Load8S(align, offset) => self.run_load_extend::<i8, i32>(context, align, offset),
			&Instruction::I32Load8U(align, offset) => self.run_load_extend::<u8, i32>(context, align, offset),
			&Instruction::I32Load16S(align, offset) => self.run_load_extend::<i16, i32>(context, align, offset),
			&Instruction::I32Load16U(align, offset) => self.run_load_extend::<u16, i32>(context, align, offset),
			&Instruction::I64Load8S(align, offset) => self.run_load_extend::<i8, i64>(context, align, offset),
			&Instruction::I64Load8U(align, offset) => self.run_load_extend::<u8, i64>(context, align, offset),
			&Instruction::I64Load16S(align, offset) => self.run_load_extend::<i16, i64>(context, align, offset),
			&Instruction::I64Load16U(align, offset) => self.run_load_extend::<u16, i64>(context, align, offset),
			&Instruction::I64Load32S(align, offset) => self.run_load_extend::<i32, i64>(context, align, offset),
			&Instruction::I64Load32U(align, offset) => self.run_load_extend::<u32, i64>(context, align, offset),

			&Instruction::I32Store(align, offset) => self.run_store::<i32>(context, align, offset),
			&Instruction::I64Store(align, offset) => self.run_store::<i64>(context, align, offset),
			&Instruction::F32Store(align, offset) => self.run_store::<F32>(context, align, offset),
			&Instruction::F64Store(align, offset) => self.run_store::<F64>(context, align, offset),
			&Instruction::I32Store8(align, offset) => self.run_store_wrap::<i32, i8>(context, align, offset),
			&Instruction::I32Store16(align, offset) => self.run_store_wrap::<i32, i16>(context, align, offset),
			&Instruction::I64Store8(align, offset) => self.run_store_wrap::<i64, i8>(context, align, offset),
			&Instruction::I64Store16(align, offset) => self.run_store_wrap::<i64, i16>(context, align, offset),
			&Instruction::I64Store32(align, offset) => self.run_store_wrap::<i64, i32>(context, align, offset),

			&Instruction::CurrentMemory(_) => self.run_current_memory(context),
			&Instruction::GrowMemory(_) => self.run_grow_memory(context),

			&Instruction::I32Const(val) => self.run_const(context, val.into()),
			&Instruction::I64Const(val) => self.run_const(context, val.into()),
			&Instruction::F32Const(val) => self.run_const(context, RuntimeValue::decode_f32(val)),
			&Instruction::F64Const(val) => self.run_const(context, RuntimeValue::decode_f64(val)),

			&Instruction::I32Eqz => self.run_eqz::<i32>(context),
			&Instruction::I32Eq => self.run_eq::<i32>(context),
			&Instruction::I32Ne => self.run_ne::<i32>(context),
			&Instruction::I32LtS => self.run_lt::<i32>(context),
			&Instruction::I32LtU => self.run_lt::<u32>(context),
			&Instruction::I32GtS => self.run_gt::<i32>(context),
			&Instruction::I32GtU => self.run_gt::<u32>(context),
			&Instruction::I32LeS => self.run_lte::<i32>(context),
			&Instruction::I32LeU => self.run_lte::<u32>(context),
			&Instruction::I32GeS => self.run_gte::<i32>(context),
			&Instruction::I32GeU => self.run_gte::<u32>(context),

			&Instruction::I64Eqz => self.run_eqz::<i64>(context),
			&Instruction::I64Eq => self.run_eq::<i64>(context),
			&Instruction::I64Ne => self.run_ne::<i64>(context),
			&Instruction::I64LtS => self.run_lt::<i64>(context),
			&Instruction::I64LtU => self.run_lt::<u64>(context),
			&Instruction::I64GtS => self.run_gt::<i64>(context),
			&Instruction::I64GtU => self.run_gt::<u64>(context),
			&Instruction::I64LeS => self.run_lte::<i64>(context),
			&Instruction::I64LeU => self.run_lte::<u64>(context),
			&Instruction::I64GeS => self.run_gte::<i64>(context),
			&Instruction::I64GeU => self.run_gte::<u64>(context),

			&Instruction::F32Eq => self.run_eq::<F32>(context),
			&Instruction::F32Ne => self.run_ne::<F32>(context),
			&Instruction::F32Lt => self.run_lt::<F32>(context),
			&Instruction::F32Gt => self.run_gt::<F32>(context),
			&Instruction::F32Le => self.run_lte::<F32>(context),
			&Instruction::F32Ge => self.run_gte::<F32>(context),

			&Instruction::F64Eq => self.run_eq::<F64>(context),
			&Instruction::F64Ne => self.run_ne::<F64>(context),
			&Instruction::F64Lt => self.run_lt::<F64>(context),
			&Instruction::F64Gt => self.run_gt::<F64>(context),
			&Instruction::F64Le => self.run_lte::<F64>(context),
			&Instruction::F64Ge => self.run_gte::<F64>(context),

			&Instruction::I32Clz => self.run_clz::<i32>(context),
			&Instruction::I32Ctz => self.run_ctz::<i32>(context),
			&Instruction::I32Popcnt => self.run_popcnt::<i32>(context),
			&Instruction::I32Add => self.run_add::<i32>(context),
			&Instruction::I32Sub => self.run_sub::<i32>(context),
			&Instruction::I32Mul => self.run_mul::<i32>(context),
			&Instruction::I32DivS => self.run_div::<i32, i32>(context),
			&Instruction::I32DivU => self.run_div::<i32, u32>(context),
			&Instruction::I32RemS => self.run_rem::<i32, i32>(context),
			&Instruction::I32RemU => self.run_rem::<i32, u32>(context),
			&Instruction::I32And => self.run_and::<i32>(context),
			&Instruction::I32Or => self.run_or::<i32>(context),
			&Instruction::I32Xor => self.run_xor::<i32>(context),
			&Instruction::I32Shl => self.run_shl::<i32>(context, 0x1F),
			&Instruction::I32ShrS => self.run_shr::<i32, i32>(context, 0x1F),
			&Instruction::I32ShrU => self.run_shr::<i32, u32>(context, 0x1F),
			&Instruction::I32Rotl => self.run_rotl::<i32>(context),
			&Instruction::I32Rotr => self.run_rotr::<i32>(context),

			&Instruction::I64Clz => self.run_clz::<i64>(context),
			&Instruction::I64Ctz => self.run_ctz::<i64>(context),
			&Instruction::I64Popcnt => self.run_popcnt::<i64>(context),
			&Instruction::I64Add => self.run_add::<i64>(context),
			&Instruction::I64Sub => self.run_sub::<i64>(context),
			&Instruction::I64Mul => self.run_mul::<i64>(context),
			&Instruction::I64DivS => self.run_div::<i64, i64>(context),
			&Instruction::I64DivU => self.run_div::<i64, u64>(context),
			&Instruction::I64RemS => self.run_rem::<i64, i64>(context),
			&Instruction::I64RemU => self.run_rem::<i64, u64>(context),
			&Instruction::I64And => self.run_and::<i64>(context),
			&Instruction::I64Or => self.run_or::<i64>(context),
			&Instruction::I64Xor => self.run_xor::<i64>(context),
			&Instruction::I64Shl => self.run_shl::<i64>(context, 0x3F),
			&Instruction::I64ShrS => self.run_shr::<i64, i64>(context, 0x3F),
			&Instruction::I64ShrU => self.run_shr::<i64, u64>(context, 0x3F),
			&Instruction::I64Rotl => self.run_rotl::<i64>(context),
			&Instruction::I64Rotr => self.run_rotr::<i64>(context),

			&Instruction::F32Abs => self.run_abs::<F32>(context),
			&Instruction::F32Neg => self.run_neg::<F32>(context),
			&Instruction::F32Ceil => self.run_ceil::<F32>(context),
			&Instruction::F32Floor => self.run_floor::<F32>(context),
			&Instruction::F32Trunc => self.run_trunc::<F32>(context),
			&Instruction::F32Nearest => self.run_nearest::<F32>(context),
			&Instruction::F32Sqrt => self.run_sqrt::<F32>(context),
			&Instruction::F32Add => self.run_add::<F32>(context),
			&Instruction::F32Sub => self.run_sub::<F32>(context),
			&Instruction::F32Mul => self.run_mul::<F32>(context),
			&Instruction::F32Div => self.run_div::<F32, F32>(context),
			&Instruction::F32Min => self.run_min::<F32>(context),
			&Instruction::F32Max => self.run_max::<F32>(context),
			&Instruction::F32Copysign => self.run_copysign::<F32>(context),

			&Instruction::F64Abs => self.run_abs::<F64>(context),
			&Instruction::F64Neg => self.run_neg::<F64>(context),
			&Instruction::F64Ceil => self.run_ceil::<F64>(context),
			&Instruction::F64Floor => self.run_floor::<F64>(context),
			&Instruction::F64Trunc => self.run_trunc::<F64>(context),
			&Instruction::F64Nearest => self.run_nearest::<F64>(context),
			&Instruction::F64Sqrt => self.run_sqrt::<F64>(context),
			&Instruction::F64Add => self.run_add::<F64>(context),
			&Instruction::F64Sub => self.run_sub::<F64>(context),
			&Instruction::F64Mul => self.run_mul::<F64>(context),
			&Instruction::F64Div => self.run_div::<F64, F64>(context),
			&Instruction::F64Min => self.run_min::<F64>(context),
			&Instruction::F64Max => self.run_max::<F64>(context),
			&Instruction::F64Copysign => self.run_copysign::<F64>(context),

			&Instruction::I32WrapI64 => self.run_wrap::<i64, i32>(context),
			&Instruction::I32TruncSF32 => self.run_trunc_to_int::<F32, i32, i32>(context),
			&Instruction::I32TruncUF32 => self.run_trunc_to_int::<F32, u32, i32>(context),
			&Instruction::I32TruncSF64 => self.run_trunc_to_int::<F64, i32, i32>(context),
			&Instruction::I32TruncUF64 => self.run_trunc_to_int::<F64, u32, i32>(context),
			&Instruction::I64ExtendSI32 => self.run_extend::<i32, i64, i64>(context),
			&Instruction::I64ExtendUI32 => self.run_extend::<u32, u64, i64>(context),
			&Instruction::I64TruncSF32 => self.run_trunc_to_int::<F32, i64, i64>(context),
			&Instruction::I64TruncUF32 => self.run_trunc_to_int::<F32, u64, i64>(context),
			&Instruction::I64TruncSF64 => self.run_trunc_to_int::<F64, i64, i64>(context),
			&Instruction::I64TruncUF64 => self.run_trunc_to_int::<F64, u64, i64>(context),
			&Instruction::F32ConvertSI32 => self.run_extend::<i32, F32, F32>(context),
			&Instruction::F32ConvertUI32 => self.run_extend::<u32, F32, F32>(context),
			&Instruction::F32ConvertSI64 => self.run_wrap::<i64, F32>(context),
			&Instruction::F32ConvertUI64 => self.run_wrap::<u64, F32>(context),
			&Instruction::F32DemoteF64 => self.run_wrap::<F64, F32>(context),
			&Instruction::F64ConvertSI32 => self.run_extend::<i32, F64, F64>(context),
			&Instruction::F64ConvertUI32 => self.run_extend::<u32, F64, F64>(context),
			&Instruction::F64ConvertSI64 => self.run_extend::<i64, F64, F64>(context),
			&Instruction::F64ConvertUI64 => self.run_extend::<u64, F64, F64>(context),
			&Instruction::F64PromoteF32 => self.run_extend::<F32, F64, F64>(context),

			&Instruction::I32ReinterpretF32 => self.run_reinterpret::<F32, i32>(context),
			&Instruction::I64ReinterpretF64 => self.run_reinterpret::<F64, i64>(context),
			&Instruction::F32ReinterpretI32 => self.run_reinterpret::<i32, F32>(context),
			&Instruction::F64ReinterpretI64 => self.run_reinterpret::<i64, F64>(context),
		}
	}

	fn run_unreachable(&mut self, _context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		Err(TrapKind::Unreachable)
	}

	fn run_nop(&mut self, _context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_block(&mut self, context: &mut FunctionContext, labels: &HashMap<usize, usize>, block_type: BlockType) -> Result<InstructionOutcome, TrapKind> {
		context.push_frame(labels, BlockFrameType::Block, block_type)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_loop(&mut self, context: &mut FunctionContext, labels: &HashMap<usize, usize>, block_type: BlockType) -> Result<InstructionOutcome, TrapKind> {
		context.push_frame(labels, BlockFrameType::Loop, block_type)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_if(&mut self, context: &mut FunctionContext, labels: &HashMap<usize, usize>, block_type: BlockType) -> Result<InstructionOutcome, TrapKind> {
		let condition: bool = context
			.value_stack_mut()
			.pop_as();
		let block_frame_type = if condition { BlockFrameType::IfTrue } else {
			let else_pos = labels[&context.position];
			if !labels.contains_key(&else_pos) {
				context.position = else_pos;
				return Ok(InstructionOutcome::RunNextInstruction);
			}

			context.position = else_pos;
			BlockFrameType::IfFalse
		};
		context.push_frame(labels, block_frame_type, block_type)?;

		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_else(&mut self, context: &mut FunctionContext, labels: &HashMap<usize, usize>) -> Result<InstructionOutcome, TrapKind> {
		let end_pos = labels[&context.position];
		context.pop_frame(false)?;
		context.position = end_pos;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_end(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		context.pop_frame(false)?;
		Ok(InstructionOutcome::End)
	}

	fn run_br(&mut self, _context: &mut FunctionContext, label_idx: u32) -> Result<InstructionOutcome, TrapKind> {
		Ok(InstructionOutcome::Branch(label_idx as usize))
	}

	fn run_br_if(&mut self, context: &mut FunctionContext, label_idx: u32) -> Result<InstructionOutcome, TrapKind> {
		let condition = context.value_stack_mut().pop_as();
		if condition {
			Ok(InstructionOutcome::Branch(label_idx as usize))
		} else {
			Ok(InstructionOutcome::RunNextInstruction)
		}
	}

	fn run_br_table(&mut self, context: &mut FunctionContext, table: &[u32], default: u32) -> Result<InstructionOutcome, TrapKind> {
		let index: u32 = context.value_stack_mut()
			.pop_as();
		Ok(InstructionOutcome::Branch(table.get(index as usize).cloned().unwrap_or(default) as usize))
	}

	fn run_return(&mut self, _context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		Ok(InstructionOutcome::Return)
	}

	fn run_call(
		&mut self,
		context: &mut FunctionContext,
		func_idx: u32,
	) -> Result<InstructionOutcome, TrapKind> {
		let func = context
			.module()
			.func_by_index(func_idx)
			.expect("Due to validation func should exists");
		Ok(InstructionOutcome::ExecuteCall(func))
	}

	fn run_call_indirect(
		&mut self,
		context: &mut FunctionContext,
		signature_idx: u32,
	) -> Result<InstructionOutcome, TrapKind> {
		let table_func_idx: u32 = context
			.value_stack_mut()
			.pop_as();
		let table = context
			.module()
			.table_by_index(DEFAULT_TABLE_INDEX)
			.expect("Due to validation table should exists");
		let func_ref = table.get(table_func_idx)
			.map_err(|_| TrapKind::TableAccessOutOfBounds)?
			.ok_or_else(|| TrapKind::ElemUninitialized)?;

		{
			let actual_function_type = func_ref.signature();
			let required_function_type = context
				.module()
				.signature_by_index(signature_idx)
				.expect("Due to validation type should exists");

			if &*required_function_type != actual_function_type {
				return Err(TrapKind::UnexpectedSignature);
			}
		}

		Ok(InstructionOutcome::ExecuteCall(func_ref))
	}

	fn run_drop(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		let _ = context
			.value_stack_mut()
			.pop();
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_select(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		let (left, mid, right) = context
			.value_stack_mut()
			.pop_triple();

		let condition = right
			.try_into()
			.expect("Due to validation stack top should be I32");
		let val = if condition { left } else { mid };
		context.value_stack_mut().push(val)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_get_local(&mut self, context: &mut FunctionContext, index: u32) -> Result<InstructionOutcome, TrapKind> {
		let val = context.get_local(index as usize);
		context.value_stack_mut().push(val)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_set_local(&mut self, context: &mut FunctionContext, index: u32) -> Result<InstructionOutcome, TrapKind> {
		let arg = context
			.value_stack_mut()
			.pop();
		context.set_local(index as usize, arg);
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_tee_local(&mut self, context: &mut FunctionContext, index: u32) -> Result<InstructionOutcome, TrapKind> {
		let arg = context
			.value_stack()
			.top()
			.clone();
		context.set_local(index as usize, arg);
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_get_global(
		&mut self,
		context: &mut FunctionContext,
		index: u32,
	) -> Result<InstructionOutcome, TrapKind> {
		let global = context
			.module()
			.global_by_index(index)
			.expect("Due to validation global should exists");
		let val = global.get();
		context.value_stack_mut().push(val)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_set_global(
		&mut self,
		context: &mut FunctionContext,
		index: u32,
	) -> Result<InstructionOutcome, TrapKind> {
		let val = context
			.value_stack_mut()
			.pop();
		let global = context
			.module()
			.global_by_index(index)
			.expect("Due to validation global should exists");
		global.set(val).expect("Due to validation set to a global should succeed");
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_load<T>(&mut self, context: &mut FunctionContext, _align: u32, offset: u32) -> Result<InstructionOutcome, TrapKind>
		where RuntimeValue: From<T>, T: LittleEndianConvert {
		let raw_address = context
			.value_stack_mut()
			.pop_as();
		let address =
			effective_address(
				offset,
				raw_address,
			)?;
		let m = context.module()
			.memory_by_index(DEFAULT_MEMORY_INDEX)
			.expect("Due to validation memory should exists");
		let b = m.get(address, mem::size_of::<T>())
			.map_err(|_| TrapKind::MemoryAccessOutOfBounds)?;
		let n = T::from_little_endian(&b)
			.expect("Can't fail since buffer length should be size_of::<T>");
		context.value_stack_mut().push(n.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_load_extend<T, U>(&mut self, context: &mut FunctionContext, _align: u32, offset: u32) -> Result<InstructionOutcome, TrapKind>
		where T: ExtendInto<U>, RuntimeValue: From<U>, T: LittleEndianConvert {
		let raw_address = context
			.value_stack_mut()
			.pop_as();
		let address =
			effective_address(
				offset,
				raw_address,
			)?;
		let m = context.module()
			.memory_by_index(DEFAULT_MEMORY_INDEX)
			.expect("Due to validation memory should exists");
		let b = m.get(address, mem::size_of::<T>())
			.map_err(|_| TrapKind::MemoryAccessOutOfBounds)?;
		let v = T::from_little_endian(&b)
			.expect("Can't fail since buffer length should be size_of::<T>");
		let stack_value: U = v.extend_into();
		context
			.value_stack_mut()
			.push(stack_value.into())
			.map_err(Into::into)
			.map(|_| InstructionOutcome::RunNextInstruction)
	}

	fn run_store<T>(&mut self, context: &mut FunctionContext, _align: u32, offset: u32) -> Result<InstructionOutcome, TrapKind>
		where T: FromRuntimeValue, T: LittleEndianConvert {
		let stack_value = context
			.value_stack_mut()
			.pop_as::<T>()
			.into_little_endian();
		let raw_address = context
			.value_stack_mut()
			.pop_as::<u32>();
		let address =
			effective_address(
				offset,
				raw_address,
			)?;

		let m = context.module()
			.memory_by_index(DEFAULT_MEMORY_INDEX)
			.expect("Due to validation memory should exists");
		m.set(address, &stack_value)
			.map_err(|_| TrapKind::MemoryAccessOutOfBounds)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_store_wrap<T, U>(
		&mut self,
		context: &mut FunctionContext,
		_align: u32,
		offset: u32,
	) -> Result<InstructionOutcome, TrapKind>
	where
		T: FromRuntimeValue,
		T: WrapInto<U>,
		U: LittleEndianConvert,
	{
		let stack_value: T = context
			.value_stack_mut()
			.pop()
			.try_into()
			.expect("Due to validation value should be of proper type");
		let stack_value = stack_value.wrap_into().into_little_endian();
		let raw_address = context
			.value_stack_mut()
			.pop_as::<u32>();
		let address =
			effective_address(
				offset,
				raw_address,
			)?;
		let m = context.module()
			.memory_by_index(DEFAULT_MEMORY_INDEX)
			.expect("Due to validation memory should exists");
		m.set(address, &stack_value)
			.map_err(|_| TrapKind::MemoryAccessOutOfBounds)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_current_memory(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		let m = context.module()
			.memory_by_index(DEFAULT_MEMORY_INDEX)
			.expect("Due to validation memory should exists");
		let s = m.current_size().0;
		context
			.value_stack_mut()
			.push(RuntimeValue::I32(s as i32))?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_grow_memory(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind> {
		let pages: u32 = context
			.value_stack_mut()
			.pop_as();
		let m = context.module()
			.memory_by_index(DEFAULT_MEMORY_INDEX)
			.expect("Due to validation memory should exists");
		let m = match m.grow(Pages(pages as usize)) {
			Ok(Pages(new_size)) => new_size as u32,
			Err(_) => u32::MAX, // Returns -1 (or 0xFFFFFFFF) in case of error.
		};
		context
			.value_stack_mut()
			.push(RuntimeValue::I32(m as i32))?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_const(&mut self, context: &mut FunctionContext, val: RuntimeValue) -> Result<InstructionOutcome, TrapKind> {
		context
			.value_stack_mut()
			.push(val)
			.map_err(Into::into)
			.map(|_| InstructionOutcome::RunNextInstruction)
	}

	fn run_relop<T, F>(&mut self, context: &mut FunctionContext, f: F) -> Result<InstructionOutcome, TrapKind>
	where
		T: FromRuntimeValue,
		F: FnOnce(T, T) -> bool,
	{
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = if f(left, right) {
			RuntimeValue::I32(1)
		} else {
			RuntimeValue::I32(0)
		};
		context.value_stack_mut().push(v)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_eqz<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where T: FromRuntimeValue, T: PartialEq<T> + Default {
		let v = context
			.value_stack_mut()
			.pop_as::<T>();
		let v = RuntimeValue::I32(if v == Default::default() { 1 } else { 0 });
		context.value_stack_mut().push(v)?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_eq<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where T: FromRuntimeValue + PartialEq<T>
	{
		self.run_relop(context, |left: T, right: T| left == right)
	}

	fn run_ne<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where T: FromRuntimeValue + PartialEq<T> {
		self.run_relop(context, |left: T, right: T| left != right)
	}

	fn run_lt<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where T: FromRuntimeValue + PartialOrd<T> {
		self.run_relop(context, |left: T, right: T| left < right)
	}

	fn run_gt<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where T: FromRuntimeValue + PartialOrd<T> {
		self.run_relop(context, |left: T, right: T| left > right)
	}

	fn run_lte<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where T: FromRuntimeValue + PartialOrd<T> {
		self.run_relop(context, |left: T, right: T| left <= right)
	}

	fn run_gte<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where T: FromRuntimeValue + PartialOrd<T> {
		self.run_relop(context, |left: T, right: T| left >= right)
	}

	fn run_unop<T, U, F>(&mut self, context: &mut FunctionContext, f: F) -> Result<InstructionOutcome, TrapKind>
	where
		F: FnOnce(T) -> U,
		T: FromRuntimeValue,
		RuntimeValue: From<U>
	{
		let v = context
			.value_stack_mut()
			.pop_as::<T>();
		let v = f(v);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_clz<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Integer<T> + FromRuntimeValue {
		self.run_unop(context, |v: T| v.leading_zeros())
	}

	fn run_ctz<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Integer<T> + FromRuntimeValue {
		self.run_unop(context, |v: T| v.trailing_zeros())
	}

	fn run_popcnt<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Integer<T> + FromRuntimeValue {
		self.run_unop(context, |v: T| v.count_ones())
	}

	fn run_add<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where RuntimeValue: From<T>, T: ArithmeticOps<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.add(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_sub<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where RuntimeValue: From<T>, T: ArithmeticOps<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.sub(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_mul<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: ArithmeticOps<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.mul(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_div<T, U>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: TransmuteInto<U> + FromRuntimeValue, U: ArithmeticOps<U> + TransmuteInto<T> {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let (left, right) = (left.transmute_into(), right.transmute_into());
		let v = left.div(right)?;
		let v = v.transmute_into();
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_rem<T, U>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: TransmuteInto<U> + FromRuntimeValue, U: Integer<U> + TransmuteInto<T> {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let (left, right) = (left.transmute_into(), right.transmute_into());
		let v = left.rem(right)?;
		let v = v.transmute_into();
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_and<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<<T as ops::BitAnd>::Output>, T: ops::BitAnd<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.bitand(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_or<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<<T as ops::BitOr>::Output>, T: ops::BitOr<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.bitor(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_xor<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<<T as ops::BitXor>::Output>, T: ops::BitXor<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.bitxor(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_shl<T>(&mut self, context: &mut FunctionContext, mask: T) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<<T as ops::Shl<T>>::Output>, T: ops::Shl<T> + ops::BitAnd<T, Output=T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.shl(right & mask);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_shr<T, U>(&mut self, context: &mut FunctionContext, mask: U) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: TransmuteInto<U> + FromRuntimeValue, U: ops::Shr<U> + ops::BitAnd<U, Output=U>, <U as ops::Shr<U>>::Output: TransmuteInto<T> {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let (left, right) = (left.transmute_into(), right.transmute_into());
		let v = left.shr(right & mask);
		let v = v.transmute_into();
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_rotl<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Integer<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.rotl(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_rotr<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Integer<T> + FromRuntimeValue
	{
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.rotr(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_abs<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue
	{
		self.run_unop(context, |v: T| v.abs())
	}

	fn run_neg<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where
		RuntimeValue: From<<T as ops::Neg>::Output>,
		T: ops::Neg + FromRuntimeValue
	{
		self.run_unop(context, |v: T| v.neg())
	}

	fn run_ceil<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue
	{
		self.run_unop(context, |v: T| v.ceil())
	}

	fn run_floor<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue
	{
		self.run_unop(context, |v: T| v.floor())
	}

	fn run_trunc<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue
	{
		self.run_unop(context, |v: T| v.trunc())
	}

	fn run_nearest<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue
	{
		self.run_unop(context, |v: T| v.nearest())
	}

	fn run_sqrt<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue
	{
		self.run_unop(context, |v: T| v.sqrt())
	}

	fn run_min<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue
	{
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.min(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_max<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.max(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_copysign<T>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<T>, T: Float<T> + FromRuntimeValue {
		let (left, right) = context
			.value_stack_mut()
			.pop_pair_as::<T>()
			.expect("Due to validation stack should contain pair of values");
		let v = left.copysign(right);
		context.value_stack_mut().push(v.into())?;
		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_wrap<T, U>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where RuntimeValue: From<U>, T: WrapInto<U> + FromRuntimeValue {
		self.run_unop(context, |v: T| v.wrap_into())
	}

	fn run_trunc_to_int<T, U, V>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
		where RuntimeValue: From<V>, T: TryTruncateInto<U, TrapKind> + FromRuntimeValue, U: TransmuteInto<V>,  {
		let v = context
			.value_stack_mut()
			.pop_as::<T>();

		v.try_truncate_into()
			.map(|v| v.transmute_into())
			.map(|v| context.value_stack_mut().push(v.into()))
			.map(|_| InstructionOutcome::RunNextInstruction)
	}

	fn run_extend<T, U, V>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where
		RuntimeValue: From<V>, T: ExtendInto<U> + FromRuntimeValue, U: TransmuteInto<V>
	{
		let v = context
			.value_stack_mut()
			.pop_as::<T>();

		let v = v.extend_into().transmute_into();
		context.value_stack_mut().push(v.into())?;

		Ok(InstructionOutcome::RunNextInstruction)
	}

	fn run_reinterpret<T, U>(&mut self, context: &mut FunctionContext) -> Result<InstructionOutcome, TrapKind>
	where
		RuntimeValue: From<U>, T: FromRuntimeValue, T: TransmuteInto<U>
	{
		let v = context
			.value_stack_mut()
			.pop_as::<T>();

		let v = v.transmute_into();
		context.value_stack_mut().push(v.into())?;

		Ok(InstructionOutcome::RunNextInstruction)
	}
}