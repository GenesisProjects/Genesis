pub const DOMAIN: &'static str = "rlp";

#[macro_use]
extern crate serde;
#[macro_use]
pub extern crate lazy_static;
pub extern crate bytebuffer;
pub extern crate gen_utils;

extern crate serde_json;

pub mod decoder;
pub mod defines;
pub mod encoder;
pub mod types;

use self::serde::ser::Serialize;
use self::serde::de::Deserialize;
use self::gen_utils::log_writer::LOGGER;

use std::convert::{Into, From};
use std::string::FromUtf8Error;
use std::net::SocketAddr;

use types::{ RLPError, RLP };

pub trait RLPSerialize: Sized {
    fn serialize(&self) -> Result<types::RLP, types::RLPError>;
    fn deserialize(rlp: &types::RLP) -> Result<Self, types::RLPError>;
}

impl<T> RLPSerialize for T
    where T: Into<RLP>
    + From<RLP>
    + Clone {
    fn serialize(&self) -> Result<types::RLP, types::RLPError> {
        Ok(self.clone().into())
    }

    fn deserialize(rlp: &RLP) -> Result<Self, types::RLPError> {
        Ok(rlp.clone().into())
    }
}

impl RLPSerialize for String {
    fn serialize(&self) -> Result<types::RLP, types::RLPError> {
        Ok(self.clone().into())
    }

    fn deserialize(rlp: &RLP) -> Result<Self, types::RLPError> {
        let result: Result<String, FromUtf8Error> = rlp.clone().into();
        Ok(result.unwrap())
    }
}