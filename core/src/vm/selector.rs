use action::Action;

use rlp::RLPSerialize;
use rlp::types::{RLPError, RLP};
use rlp::decoder::Decoder;
use rlp::encoder::Encoder;

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
        let name_rlp: RLP = self.name.to_owned().into();

        let mut args_rlp: RLP = rlp_list![];
        for arg in self.args.clone() {
            match arg {
                Argument::Int32(value) => {
                    let type_item: RLP = "i32".to_owned().into();
                    args_rlp = args_rlp << type_item;
                    let arg_item: RLP = (value as u32).into();
                    args_rlp = args_rlp << arg_item;
                },
                Argument::Int64(value) => {
                    let type_item: RLP = "i64".to_owned().into();
                    args_rlp = args_rlp << type_item;
                    let arg_item: RLP = (value as u64).into();
                    args_rlp = args_rlp << arg_item;
                },
                Argument::Uint32(value) => {
                    let type_item: RLP = "u32".to_owned().into();
                    args_rlp = args_rlp << type_item;
                    let arg_item: RLP = (value as u32).into();
                    args_rlp = args_rlp << arg_item;
                },
                Argument::Uint64(value) => {
                    let type_item: RLP = "f32".to_owned().into();
                    args_rlp = args_rlp << type_item;
                    let arg_item: RLP = (value as u64).into();
                    args_rlp = args_rlp << arg_item;
                },
                Argument::Float32(value) => {
                    let type_item: RLP = "f64".to_owned().into();
                    args_rlp = args_rlp << type_item;
                    let arg_item: RLP = (value as u32).into();
                    args_rlp = args_rlp << arg_item;
                },
                Argument::Float64(value) => {
                    let type_item: RLP = "u64".to_owned().into();
                    args_rlp = args_rlp << type_item;
                    let arg_item: RLP = (value as u64).into();
                    args_rlp = args_rlp << arg_item;
                }
            }
        }

        let mut returns_rlp: RLP = rlp_list![];
        for ret in self.returns.clone() {
            match ret {
                Argument::Int32(value) => {
                    let type_item: RLP = "i32".to_owned().into();
                    returns_rlp = returns_rlp << type_item;
                    let ret_item: RLP = (value as u32).into();
                    returns_rlp = returns_rlp << ret_item;
                },
                Argument::Int64(value) => {
                    let type_item: RLP = "i64".to_owned().into();
                    returns_rlp = returns_rlp << type_item;
                    let ret_item: RLP = (value as u64).into();
                    returns_rlp = returns_rlp << ret_item;
                },
                Argument::Uint32(value) => {
                    let type_item: RLP = "u32".to_owned().into();
                    returns_rlp = returns_rlp << type_item;
                    let ret_item: RLP = (value as u32).into();
                    returns_rlp = returns_rlp << ret_item;
                },
                Argument::Uint64(value) => {
                    let type_item: RLP = "u64".to_owned().into();
                    returns_rlp = returns_rlp << type_item;
                    let ret_item: RLP = (value as u64).into();
                    returns_rlp = returns_rlp << ret_item;
                },
                Argument::Float32(value) => {
                    let type_item: RLP = "f64".to_owned().into();
                    returns_rlp = returns_rlp << type_item;
                    let ret_item: RLP = (value as u32).into();
                    returns_rlp = returns_rlp << ret_item;
                },
                Argument::Float64(value) => {
                    let type_item: RLP = "u64".to_owned().into();
                    returns_rlp = returns_rlp << type_item;
                    let ret_item: RLP = (value as u64).into();
                    returns_rlp = returns_rlp << ret_item;
                }
            }

        }

        Ok(rlp_list![
            name_rlp,
            args_rlp,
            returns_rlp
        ])
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
        let rlp = Decoder::decode(&input.to_vec()).unwrap();
        Selector::deserialize(&rlp)
    }

    fn encode<'a>(&self, buff: &mut[u8]) {
        let mut encoder = Encoder::new();
        let rlp: RLP = self.serialize().unwrap();
        let result = encoder.encode(&rlp);
        buff.copy_from_slice(&result[..]);
    }
}