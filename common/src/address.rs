use std::string::String;

use key::*;
use rlp::RLPSerialize;
use rlp::types::*;
use rust_base58::{ToBase58, FromBase58};
use std::borrow::Borrow;

/// Common key operations
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct Address {
    pub text: String
}

impl Address {
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
        Some(Address {text: "12345678901234567890123456789012".to_string()})
    }

    /// load vec
    pub fn try_from(value: Vec<u8>) -> Result<Self, ()> {
        match String::from_utf8(value) {
            Ok(r) => Ok(Address {text: r}),
            Err(_) => Err(())
        }
    }
}

impl Borrow<str> for Address {
    fn borrow(&self) -> &str {
        &self.text
    }
}

impl From<PublicKey> for Address {
    fn from(v: PublicKey) -> Self {
        Address {
            text: v.to_base58()
        }
    }
}

impl RLPSerialize for Address {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(RLP::RLPItem(self.text.to_owned().into()))
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        use std::str;
        match rlp {
            &RLP::RLPItem(ref value) => {
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