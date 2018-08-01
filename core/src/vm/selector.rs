use action::Action;
use std::panic;

use rlp::decoder::Decoder;
use rlp::encoder::Encoder;
use rlp::RLPSerialize;
use rlp::types::*;

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

    pub fn decode(input: &Vec<u8>) -> Option<Selector> {
        match Decoder::decode(input) {
            Some(r) => {
                match Selector::deserialize(&r) {
                    Ok(r) => Some(r),
                    _ => None
                }
            },
            _ => None
        }
    }

    pub fn encode<'a>(&self) -> Result<EncodedRLP, &'static str> {
        let mut encoder = Encoder::new();
        match self.serialize() {
            Ok(r) => {
                let result = encoder.encode(&r);
                Ok(result)
            },
            _ => Err("rlp serialization failed")
        }

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
        let result = panic::catch_unwind(|| {
            let name = String::deserialize(&rlp[0]).unwrap();
            let mut args: Vec<Argument> = vec![];
            let mut returns: Vec<Argument> = vec![];

            if let RLP::RLPList(ref arg_list) = rlp[1] {
                let mut i = 0usize;
                while i < arg_list.len() {
                    match String::deserialize(&arg_list[i]).unwrap().as_ref() {
                        "i32" => args.push(Argument::Int32(arg_list[i + 1].to_owned().into())),
                        "i64" => args.push(Argument::Int64(arg_list[i + 1].to_owned().into())),
                        "u32" => args.push(Argument::Uint32(arg_list[i + 1].to_owned().into())),
                        "u64" => args.push(Argument::Uint64(arg_list[i + 1].to_owned().into())),
                        "f32" => args.push(Argument::Float32(arg_list[i + 1].to_owned().into())),
                        "f64" => args.push(Argument::Float64(arg_list[i + 1].to_owned().into())),
                        _ => panic!("Unknown data type")
                    }
                    i += 2;
                }
            } else {
                panic!("Unknown args")
            }

            if let RLP::RLPList(ref ret_list) = rlp[2] {
                let mut i = 0usize;
                while i < ret_list.len() {
                    match String::deserialize(&ret_list[i]).unwrap().as_ref() {
                        "i32" => returns.push(Argument::Int32(ret_list[i + 1].to_owned().into())),
                        "i64" => returns.push(Argument::Int64(ret_list[i + 1].to_owned().into())),
                        "u32" => returns.push(Argument::Uint32(ret_list[i + 1].to_owned().into())),
                        "u64" => returns.push(Argument::Uint64(ret_list[i + 1].to_owned().into())),
                        "f32" => returns.push(Argument::Float32(ret_list[i + 1].to_owned().into())),
                        "f64" => returns.push(Argument::Float64(ret_list[i + 1].to_owned().into())),
                        _ => panic!("Unknown data type")
                    }
                    i += 2;
                }
            } else {
                panic!("Unknown args")
            }

            Selector {
                name: name,
                args: args,
                returns: returns
            }
        });

        match result {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{:?}", e);
                Err(RLPError::RLPDecodingErrorMalformed)
            }
        }
    }
}