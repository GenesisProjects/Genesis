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

impl RLPSerialize for u8 {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(RLP::RLPItem(vec![self.clone()]))
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        match rlp {
            &RLP::RLPItem (ref value) => Ok(value[0]),
            _ => Err(RLPError::RLPErrorType)
        }
    }
}

impl RLPSerialize for u16 {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(RLP::RLPItem(vec![(self >> 8) as u8, (self & 0x00ff) as u8]))
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        match rlp {
            &RLP::RLPItem(ref value) => Ok(((value[0] as u16) << 8) + value[1] as u16),
            _ => Err(RLPError::RLPErrorType),
        }
    }
}

impl RLPSerialize for String {
    fn serialize(&self) -> Result<types::RLP, types::RLPError> {
        Ok(RLP::RLPItem(self.as_bytes().to_vec()))
    }
    fn deserialize(rlp: &types::RLP) -> Result<Self, types::RLPError> {
        match rlp {
            &RLP::RLPItem(ref value) => match String::from_utf8(value.to_owned()) {
                Ok(str) => Ok(str),
                Err(_) => Err(RLPError::RLPErrorUnknown)
            }
            _ => Err(RLPError::RLPErrorUnknown)
        }
    }
}

impl RLPSerialize for SocketAddr {
    fn serialize(&self) -> Result<types::RLP, types::RLPError> {
        let ip_rlp = RLPSerialize::serialize(&self.ip().to_string()).unwrap_or(RLP::RLPEmpty);
        let port_rlp = RLPSerialize::serialize(&self.port()).unwrap_or(RLP::RLPEmpty);
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