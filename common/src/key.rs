use address::*;
use ring::{rand, signature};
use ring::signature::ED25519_PKCS8_V2_LEN as PKCS_LEN;
use untrusted::Input as Input;

pub const PUBLIC_KEY_LEN: usize = 32;

pub type PublicKey = [u8; PUBLIC_KEY_LEN];
pub type Secret = [u8; PKCS_LEN];

pub type Signature = signature::Signature;

/// KeyPair which store public key and secret
pub struct KeyPair {
    pair: signature::Ed25519KeyPair
}

/// Common keypair operation
pub trait KeyPairOp<'a, 'b> {
    /// Get the public key
    fn public_key_str(&'a self) -> PublicKey;

    /// Get the public key
    fn gen_rand_keypair() -> Result<(KeyPair, Secret), String>;
    fn restore_keypair(input: &'a [u8]) -> Result<KeyPair, String>;
    fn restore_keypair_from_file() -> Result<KeyPair, String>;

    fn sign_msg(&self, msg: &'a [u8]) -> signature::Signature;
    fn verify_sig(addr: &Address, msg: &'a [u8], sig: &signature::Signature) -> bool;
}

impl<'a, 'b> KeyPairOp<'a, 'b> for KeyPair {
    #[inline]
    fn public_key_str(&'a self) -> PublicKey {
        let mut a: PublicKey = Default::default();
        a.copy_from_slice(self.pair.public_key_bytes());
        a
    }

    #[inline]
    fn gen_rand_keypair() -> Result<(KeyPair, Secret), String> {
        let rng = rand::SystemRandom::new();
        match signature::Ed25519KeyPair::generate_pkcs8(&rng) {
            Err(why) => {
                Err(String::from(format!("{}{}", "Failed to generate pkcs8: ", why)))
            }
            Ok(pkcs8_bytes) =>
                match signature::Ed25519KeyPair::from_pkcs8(Input::from(&pkcs8_bytes)) {
                    Err(why) => {
                        Err(String::from(format!("{}{}", "Failed to generate key_pair: ", why)))
                    }
                    Ok(key_pair) => {
                        Ok((KeyPair { pair: key_pair }, pkcs8_bytes))
                    }
                }
        }
    }

    #[inline]
    fn restore_keypair(input: &'a [u8]) -> Result<KeyPair, String> {
        match signature::Ed25519KeyPair::from_pkcs8(Input::from(&input)) {
            Err(why) => {
                Err(String::from(format!("{}{}", "Failed to generate key_pair: ", why)))
            }
            Ok(key_pair) => {
                Ok(KeyPair { pair: key_pair })
            }
        }
    }

    #[inline]
    fn restore_keypair_from_file() -> Result<KeyPair, String> {
        //TODO:
        let input: Vec<u8> = vec![1,2,3];
        match signature::Ed25519KeyPair::from_pkcs8(Input::from(&input[..])) {
            Err(why) => {
                Err(String::from(format!("{}{}", "Failed to generate key_pair: ", why)))
            }
            Ok(key_pair) => {
                Ok(KeyPair { pair: key_pair })
            }
        }
    }

    #[inline]
    fn sign_msg(&self, msg: &'a [u8]) -> signature::Signature {
        self.pair.sign(msg)
    }

    #[inline]
    fn verify_sig(addr: &Address, msg: &'a [u8], sig: &signature::Signature) -> bool {
        let peer_public_key_bytes = addr.to_key().unwrap();
        let sig_bytes = sig.as_ref();

        let peer_public_key = Input::from(&peer_public_key_bytes);
        let msg = Input::from(msg);
        let sig = Input::from(sig_bytes);

        match signature::verify(&signature::ED25519, peer_public_key, msg, sig) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_key_pair() {}
}