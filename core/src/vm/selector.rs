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
    pub fn new(name: String, args: Vec<Argument>, returns: Vec<Argument>) -> Self {
        Selector {
            name: name,
            args: args,
            returns: returns
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn args(&self) -> Vec<RuntimeValue> {
        self.args.clone().into_iter().map(|arg| {
            match arg {
                Argument::Int32(val) => RuntimeValue::from(val),
                Argument::Int64(val) => RuntimeValue::from(val),
                Argument::Uint32(val) => RuntimeValue::from(val),
                Argument::Uint64(val) => RuntimeValue::from(val),
                Argument::Float32(val) => RuntimeValue::F32(val.into()),
                Argument::Float64(val) => RuntimeValue::F64(val.into())
            }
        }).collect()
    }
}

impl From<Action> for Selector {
    fn from(f: Action) -> Self {
        //TODO: test
        //unimplemented!()
        Selector{
            name: "test".to_string(),
            args: vec![],
            returns: vec![]
        }
    }
}

impl RLPSerialize for Selector {
    fn serialize(&self) -> Result<RLP, RLPError> {
        // [
        //      "test_func",                            # function name
        //      [["i32"],["100"],["string"],["abc"]     # inputs
        //      [["bool"]]                              # outputs
        // ]
        let name_rlp = RLPSerialize::serialize(&self.name).unwrap_or(RLP::RLPEmpty);

        let mut args_rlp: Vec<RLP> = vec![];
        for arg in self.args {
            let arg_item = RLPSerialize::serialize(&arg).unwrap_or(RLP::RLPEmpty);
            args_rlp.append(&mut vec![arg_item]);
        }

        let mut returns_rlp: Vec<RLP> = vec![];
        for ret in self.returns {
            let ret_item = RLPSerialize::serialize(&ret).unwrap_or(RLP::RLPEmpty);
            returns_rlp.append(&mut vec![ret_item]);
        }

        Ok(RLP::RLPList { list: vec![name_rlp, args_rlp, returns_rlp] })
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