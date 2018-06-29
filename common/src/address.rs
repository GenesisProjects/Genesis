extern crate ring;
extern crate rust_base58;

use std::string::String;

use rlp::RLPSerialize;
use rlp::types::*;

use self::rust_base58::{ToBase58, FromBase58};
use super::key::*;

/// Common key operations
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Address {
    pub text: String
}

impl Address {
    /// Convert key to addr
    pub fn key2addr(key: PublicKey) -> Address {
        Address { text: key.to_base58() }
    }

    /// Convert to key
    pub fn to_key(&self) -> Option<PublicKey> {
        match self.text.from_base58() {
            Ok(r) => {
                if r.len() != PUBLIC_KEY_LEN {
                    None
                } else {
                    let mut a: [u8; 32] = Default::default();
                    a.copy_from_slice(&r[0..PUBLIC_KEY_LEN]);
                    Some(a)
                }
            },
            Err(_) => None
        }
    }

    /// load account
    pub fn load() -> Option<Self> {
        unimplemented!()
    }
}

impl RLPSerialize for Address {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(RLP::RLPItem { value: self.text.to_owned().into() })
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        use std::str;
        match rlp {
            &RLP::RLPItem { ref value } => {
                Ok(Address { text: str::from_utf8(value).unwrap().into() })
            },
            _ => {
                Err(RLPError::RLPErrorType)
            }
        }
    }
}

#[cfg(test)]
mod tests {

}