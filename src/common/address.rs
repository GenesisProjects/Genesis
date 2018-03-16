extern crate ring;
extern crate rust_base58;

use self::ring::signature::ED25519_PKCS8_V2_LEN as PKCS_LEN;
use std::string::String;

use self::rust_base58::{ToBase58, FromBase58};

pub const PUBLIC_KEY_LEN: usize = 32;
pub type Address = [u8; PUBLIC_KEY_LEN];
pub type Secret = [u8; PKCS_LEN];

/// Common key operations
pub trait AddressOp {
    /// Cast public key to string
    fn addr2string(&self) -> String;

    /// Cast public key from string
    fn string2addr(input: &str) -> Option<Address>;
}

impl AddressOp for Address {
    fn addr2string(&self) -> String {
        self.to_base58()
    }

    fn string2addr(input: &str) -> Option<Address> {
        match input.from_base58() {
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

impl AddressOp for Secret {
    fn addr2string(&self) -> String {
        self.to_base58()
    }

    fn string2addr(input: &str) -> Option<Address> {
        match input.from_base58() {
            Ok(r) => {
                if r.len() != PKCS_LEN {
                    None
                } else {
                    let mut a: [u8; 32] = Default::default();
                    a.copy_from_slice(&r[0..PKCS_LEN]);
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