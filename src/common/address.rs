extern crate ring;
extern crate rust_base58;

use std::string::String;
use self::rust_base58::{ToBase58, FromBase58};

use super::key::*;

/// Common key operations
#[derive(Debug, Clone)]
pub struct Address {
    pub text: String
}

impl Address {
    /// Convert key to addr
    pub fn key2addr(key: PublicKey) -> String {
        key.to_base58()
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
            Err(e) => None
        }
    }
}

#[cfg(test)]
mod tests {

}