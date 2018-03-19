extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use std::string::String;

/// macro gen_hash! takes (*_str => '&str' type data) and (*_raw => '&[u8]' type data) as input
/// it generates a 'String' type output
#[macro_export]
macro_rules! gen_hash {
    (sha256_str => $e:expr) => ({
        let mut sha = Sha256::new();
        sha.input_str($e);
        sha.result_str()
    });
    (sha256_raw => $e:expr) => ({
        let mut sha = Sha256::new();
        sha.input($e);
        sha.result_str()
    })
}

/// Hash structure
#[derive(Debug)]
pub struct Hash {
    data: String,
    len: u32,
    note: String
}

/// Interface for hashable objects
pub trait SHA256Hashable {
    fn serialized_data(&self) -> Vec<u8>;

    #[inline]
    fn encrype_sha256(&self) -> Hash {
        let data = gen_hash!(sha256_raw => &self.serialized_data());
        Hash { data: data, len: 32, note: String::from("SHA256") }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encrype() {}
}