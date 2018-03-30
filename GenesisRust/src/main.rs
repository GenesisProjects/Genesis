extern crate common;
extern crate gen_core;
extern crate rlp;

use self::rlp::encoder::Encoder;
use self::rlp::decoder::Decoder;
use self::rlp::types::RLP;
use self::rlp::RLPSerialize;

use self::gen_core::mpt::node::*;

fn main() {
    let test = TrieNode::LeafNode { encoded_path: vec![65u8, 65u8], value: "test".to_string() };
    match test.serialize() {
        Ok(r) => {
            print!("{:?}\n", r);
            let a = TrieNode::<String>::deserialize(&r);
            match a {
                Ok(r) => { print!("{:?}\n", r); },
                Err(e) => {  }
            }
        }
        Err(e) => {  }
    };

}