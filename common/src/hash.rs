use crypto::digest::Digest;
use crypto::sha2::Sha256;

use rlp::RLPSerialize;
use rlp::encoder::SHARED_ENCODER;
use rlp::types::RLP;
use rlp::encoder::Encoder;

/// macro gen_hash! takes (*_str => '&str' type data) and (*_raw => '&[u8]' type data) as input
/// it generates a 'String' type output

/// Hash lenth
pub const HASH_LEN: usize = 32usize;

/// Hash structure
pub type Hash = [u8; HASH_LEN];

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
        [0u8; HASH_LEN]
    });
}

#[macro_export]
macro_rules! hash_len {
    ($e:expr) => ({
        $e.len()
    });
}

/// Interface for hashable objects
pub trait SerializableAndSHA256Hashable: RLPSerialize {
    #[inline]
    fn encrype_sha256(&self) -> Option<(Hash, Vec<u8>)>;
}

impl<T> SerializableAndSHA256Hashable for T where T: RLPSerialize {
    #[inline]
    fn encrype_sha256(&self) -> Option<(Hash, Vec<u8>)> {
        match self.serialize() {
            Ok(r) => {
                let encoded_rlp = SHARED_ENCODER.lock().unwrap().encode(&r);
                let data: String = gen_hash!(sha256_raw => &encoded_rlp);
                let mut result: Hash = [0; HASH_LEN];
                result.clone_from_slice(&data.as_bytes()[0 .. HASH_LEN]);
                Some((result, encoded_rlp))
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