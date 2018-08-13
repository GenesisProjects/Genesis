pub const DOMAIN: &'static str = "rlp";

#[macro_use]
pub extern crate lazy_static;
pub extern crate bytebuffer;
pub extern crate gen_utils;

extern crate serde_json;

pub mod decoder;
pub mod defines;
pub mod encoder;
pub mod types;

use self::gen_utils::log_writer::LOGGER;

use std::convert::{Into, From};
use std::string::FromUtf8Error;

use types::{ RLPError, RLP };

pub trait RLPSerialize: Sized {
    fn serialize(&self) -> Result<RLP, RLPError>;
    fn deserialize(rlp: &types::RLP) -> Result<Self, RLPError>;
}

impl<T> RLPSerialize for T
    where T: Into<RLP>
    + From<RLP>
    + Clone {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(self.clone().into())
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        Ok(rlp.clone().into())
    }
}

impl RLPSerialize for String {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(self.clone().into())
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        let result: Result<String, FromUtf8Error> = rlp.clone().into();
        Ok(result.unwrap())
    }
}