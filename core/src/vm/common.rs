use std::collections::VecDeque;
use std::error;
use std::fmt;
use parity_wasm::elements::BlockType;

pub mod stack {
	#[derive(Debug)]
	pub struct Error(String);

	impl fmt::Display for Error {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{}", self.0)
		}
	}

	impl error::Error for Error {
		fn description(&self) -> &str {
			&self.0
		}
	}

	/// Stack with limit.
	#[derive(Debug)]
	pub struct StackWithLimit<T> where T: Clone {
		/// Stack values.
		values: VecDeque<T>,
		/// Stack limit (maximal stack len).
		limit: usize,
	}

	impl<T> StackWithLimit<T> where T: Clone {
		pub fn with_limit(limit: usize) -> Self {
			StackWithLimit {
				values: VecDeque::new(),
				limit: limit
			}
		}

		pub fn is_empty(&self) -> bool {
			self.values.is_empty()
		}

		pub fn len(&self) -> usize {
			self.values.len()
		}

		pub fn limit(&self) -> usize {
			self.limit
		}

		pub fn top(&self) -> Result<&T, Error> {
			self.values
				.back()
				.ok_or_else(|| Error("non-empty stack expected".into()))
		}

		pub fn top_mut(&mut self) -> Result<&mut T, Error> {
			self.values
				.back_mut()
				.ok_or_else(|| Error("non-empty stack expected".into()))
		}

		pub fn get(&self, index: usize) -> Result<&T, Error> {
			if index >= self.values.len() {
				return Err(Error(format!("trying to get value at position {} on stack of size {}", index, self.values.len())));
			}

			Ok(self.values.get(self.values.len() - 1 - index).expect("checked couple of lines above"))
		}

		pub fn push(&mut self, value: T) -> Result<(), Error> {
			if self.values.len() >= self.limit {
				return Err(Error(format!("exceeded stack limit {}", self.limit)));
			}

			self.values.push_back(value);
			Ok(())
		}

		pub fn pop(&mut self) -> Result<T, Error> {
			self.values
				.pop_back()
				.ok_or_else(|| Error("non-empty stack expected".into()))
		}

		pub fn resize(&mut self, new_size: usize, dummy: T) {
			debug_assert!(new_size <= self.values.len());
			self.values.resize(new_size, dummy);
		}
	}
}

/// Index of default linear memory.
pub const DEFAULT_MEMORY_INDEX: u32 = 0;
/// Index of default table.
pub const DEFAULT_TABLE_INDEX: u32 = 0;

/// Control stack frame.
#[derive(Debug, Clone)]
pub struct BlockFrame {
	/// Frame type.
	pub frame_type: BlockFrameType,
	/// A signature, which is a block signature type indicating the number and types of result values of the region.
	pub block_type: BlockType,
	/// A label for reference to block instruction.
	pub begin_position: usize,
	/// A label for reference from branch instructions.
	pub branch_position: usize,
	/// A label for reference from end instructions.
	pub end_position: usize,
	/// A limit integer value, which is an index into the value stack indicating where to reset it to on a branch to that label.
	pub value_stack_len: usize,
	/// Boolean which signals whether value stack became polymorphic. Value stack starts in non-polymorphic state and
	/// becomes polymorphic only after an instruction that never passes control further is executed,
	/// i.e. `unreachable`, `br` (but not `br_if`!), etc.
	pub polymorphic_stack: bool,
}

/// Type of block frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockFrameType {
	/// Function frame.
	Function,
	/// Usual block frame.
	Block,
	/// Loop frame (branching to the beginning of block).
	Loop,
	/// True-subblock of if expression.
	IfTrue,
	/// False-subblock of if expression.
	IfFalse,
}