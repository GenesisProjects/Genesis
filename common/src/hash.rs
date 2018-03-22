extern crate crypto;
extern crate rlp;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

use self::rlp::RLPSerialize;
use self::rlp::types::RLP;

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
pub trait SHA256Hashable<'a>: RLPSerialize<'a> {
    #[inline]
    fn encrype_sha256(&self) -> Option<Hash> {
        match self.encode() {
            Ok(r) => {
                let data = gen_hash!(sha256_raw => &r);
                Some(Hash { data: data, len: 32, note: String::from("SHA256") })
            },
            Err(_) => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encrype() {}
}