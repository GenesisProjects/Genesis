#[macro_use]
extern crate serde;

#[macro_use]
pub extern crate lazy_static;

extern crate serde_json;

pub mod decoder;
pub mod defines;
pub mod encoder;
pub mod types;

use self::serde::ser::Serialize;
use self::serde::de::Deserialize;

use types::{ RLPError, RLP };

pub trait RLPSerialize: Sized {
    fn serialize(&self) -> Result<types::RLP, types::RLPError>;
    fn deserialize(rlp: &types::RLP) -> Result<Self, types::RLPError>;
}

impl RLPSerialize for String {
    fn serialize(&self) -> Result<types::RLP, types::RLPError> {
        Ok(RLP::RLPItem { value: self.as_bytes().to_vec() })
    }
    fn deserialize(rlp: &types::RLP) -> Result<Self, types::RLPError> {
        match rlp {
            &RLP::RLPItem { ref value } => match String::from_utf8(value.clone()) {
                Ok(str) => Ok(str),
                Err(_) => Err(RLPError::RLPErrorUnknown)
            }
            _ => Err(RLPError::RLPErrorUnknown)
        }
    }
}