use common::hash::Hash;
use common::address::Address;
use rlp::RLPSerialize;
use rlp::types::{RLPError, RLP};


#[derive(Clone, Debug)]
pub enum Argument {
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
    String(Vec<u8>),
    Bool(i8),
    Hash(Hash),
    Account(Address)
}

#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    args: Vec<Argument>,
    returns: Vec<Argument>
}

impl From<Function> for String {
    fn from(f: Function) -> Self {
        unimplemented!()
    }
}

impl RLPSerialize for Function {
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

pub trait ABICodec {
    fn decode(input: &[u8]) -> Self;
    fn encode<'a>(&self, buff: &mut[u8]);
}

impl<T> ABICodec for T where T: RLPSerialize {
    fn decode(input: &[u8]) -> T {
        unimplemented!()
    }

    fn encode<'a>(&self, buff: &mut[u8]) {
        unimplemented!()
    }
}