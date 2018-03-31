extern crate common;
extern crate gen_core;
extern crate rlp;

use self::rlp::encoder::{ Encoder, SHARED_ENCODER };
use self::rlp::decoder::Decoder;
use self::rlp::types::RLP;
use self::rlp::RLPSerialize;

use self::gen_core::mpt::node::*;

fn main() {
    let test = TrieNode::ExtensionNode::<String> { encoded_path: vec![05u8, 65u8], key: [0u8; 32] };
    match test.serialize() {
        Ok(r) => {
            print!("{:?}\n", SHARED_ENCODER.lock().unwrap().encode(&r));
            let a = TrieNode::<String>::deserialize(&r);
            match a {
                Ok(r) => { print!("{:?}\n", r); },
                Err(e) => {  }
            }
        }
        Err(e) => {  }
    };

}