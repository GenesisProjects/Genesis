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

use std::net::SocketAddr;

use types::{ RLPError, RLP };

pub trait RLPSerialize: Sized {
    fn serialize(&self) -> Result<types::RLP, types::RLPError>;
    fn deserialize(rlp: &types::RLP) -> Result<Self, types::RLPError>;
}

impl RLPSerialize for SocketAddr {
    fn serialize(&self) -> Result<types::RLP, types::RLPError> {
        let ip_rlp: RLP = self.ip().to_string().into();
        let port_rlp: RLP = self.port().into();
        Ok(RLP::RLPList(vec![ip_rlp, port_rlp]))
    }
    fn deserialize(rlp: &types::RLP) -> Result<Self, types::RLPError> {
        /*match rlp {
            &RLP::RLPItem { ref value } => match String::from_utf8(value.to_owned()) {
                Ok(str) => Ok(str),
                Err(_) => Err(RLPError::RLPErrorUnknown)
            }
            _ => Err(RLPError::RLPErrorUnknown)
        }*/
        unimplemented!()
    }
}