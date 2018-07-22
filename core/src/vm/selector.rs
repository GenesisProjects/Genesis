use action::Action;

use common::hash::Hash;
use common::address::Address;

use rlp::RLPSerialize;
use rlp::types::{RLPError, RLP};

use wasmi::*;

#[derive(Clone, Debug)]
pub enum Argument {
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Float32(f32),
    Float64(f64)
}

#[derive(Clone, Debug)]
pub struct Selector {
    name: String,
    args: Vec<Argument>,
    returns: Vec<Argument>
}

impl Selector {
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn args(&self) -> Vec<RuntimeValue> {
        unimplemented!()
    }
}

impl From<Action> for Selector {
    fn from(f: Action) -> Self {
        unimplemented!()
    }
}

impl RLPSerialize for Selector {
    fn serialize(&self) -> Result<RLP, RLPError> {
        // [
        //      "test_func",                            # function name
        //      [["i32"],["100"],["string"],["abc"]     # inputs
        //      [["bool"]]                              # outputs
        // ]
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

pub trait SelectorCodec {
    fn decode(input: &[u8]) -> Self;
    fn encode<'a>(&self, buff: &mut[u8]);
}

impl<T> SelectorCodec for T where T: RLPSerialize {
    fn decode(input: &[u8]) -> T {
        unimplemented!()
    }

    fn encode<'a>(&self, buff: &mut[u8]) {
        unimplemented!()
    }
}