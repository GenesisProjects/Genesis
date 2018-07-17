use exceptions::trap::*;
use types::*;

pub struct Memory {
    bytes: Vec<u8>,

    current_size: usize,
}

impl Memory {
    /// init a memory instance with num of pages
    pub fn new(&self, pages: usize) -> Result<Self, Trap> {
        unimplemented!()
    }

    /// grow linear memory by a given unsigned delta of pages.
    /// Return the previous memory size in units of pages or [[Trap]].
    pub fn grow_memory(&self, pages: usize) -> Result<usize, Trap> {
        self.access(0, 0);
        unimplemented!()
    }

    pub fn access(&self, base: u32, offset: u32) -> Result<&[u8], Trap> {
        unimplemented!()
    }
}