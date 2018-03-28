#[macro_use]
extern crate serde;
extern crate serde_json;

pub mod decoder;
pub mod defines;
pub mod encoder;
pub mod types;

use self::serde::ser::Serialize;
use self::serde::de::Deserialize;

pub trait RLPSerialize: Sized {
    fn encode(&self) -> Result<types::EncodedRLP, types::RLPError>;
    fn decode(encoded_rlp: &types::EncodedRLP) -> Result<Self, types::RLPError>;
}