extern crate crypto;
extern crate rlp;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

use self::rlp::RLPSerialize;
use self::rlp::encoder::SHARED_ENCODER;
use self::rlp::types::RLP;
use self::rlp::encoder::Encoder;

/// macro gen_hash! takes (*_str => '&str' type data) and (*_raw => '&[u8]' type data) as input
/// it generates a 'String' type output

/// Hash structure
pub type Hash = [u8; 32];

/// Empty Hash
pub const ZERO_HASH: [u8; 32] = [0; 32];

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

#[macro_export]
macro_rules! zero_hash {
    () => ({
        ZERO_HASH
    });
}

#[macro_export]
macro_rules! equal_hash {
    ($e:expr) => ({
        $e.eq(ZERO_HASH);
    });
}

#[macro_export]
macro_rules! hash_len {
    ($e:expr) => ({
        $e.len()
    });
}



/// Interface for hashable objects
pub trait SerializableAndSHA256Hashable<'a>: RLPSerialize {
    #[inline]
    fn encrype_sha256(&self) -> Option<Hash> {
        match self.serialize() {
            Ok(r) => {
                let encoded_rlp = SHARED_ENCODER.lock().unwrap().encode(&r);
                let data: String = gen_hash!(sha256_raw => &encoded_rlp);
                let mut result: Hash = [0; 32];
                result.clone_from_slice(&data.as_bytes()[0..32]);
                Some(result)
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