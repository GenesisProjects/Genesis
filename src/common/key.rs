extern crate ring;
extern crate untrusted;

use self::ring::{rand, signature};

pub enum KeyPair {
    Ed25519KeyPair { pair: signature::Ed25519KeyPair },
    ECDSAKeyPair { pair: signature::ECDSAKeyPair },
}

trait KeyPairOp {
    fn public_key_str(&self) -> String;
    fn private_key_str(&self) -> String;

    fn gen_ed25519_keypair() -> Result<KeyPair, String>;
    fn gen_ecdsakey_pair_keypair() -> Result<KeyPair, String>;

    fn sign_msg(msg: & [u8]) -> signature::Signature;
    fn verify_sig(msg: & [u8], sig: &signature::Signature, pub_key: &String) -> bool;
}

impl KeyPairOp for KeyPair {
    fn gen_ed25519_keypair() -> Result<KeyPair, String> {
        let rng = rand::SystemRandom::new();
        match signature::Ed25519KeyPair::generate_pkcs8(&rng) {
            Err(why) => {
                panic!("{:?}", why);
                Err(String::from("Failed to generate pkcs8: " + why))
            },
            Ok(pkcs8_bytes) => match signature
            ::Ed25519KeyPair
            ::from_pkcs8(untrusted::Input::from(&pkcs8_bytes)) {
                Err(why) => {
                    panic!("{:?}", why);
                    Err(String::from("Failed to generate key_pair: " + why))
                },
                Ok(key_pair) => Ok(Ed25519KeyPair { pair: key_pair })
            },
        }
    }

    // TODO::
}

/*
impl KeyPairOp for KeyPair:: ECDSAKeyPair {

}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn create_key_pair() {}
}