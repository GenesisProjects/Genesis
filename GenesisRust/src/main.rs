extern crate common;
extern crate gen_core;
extern crate rlp;

use self::rlp::encoder::Encoder;
use self::rlp::decoder::Decoder;
use self::rlp::types::RLP;

use self::gen_core::mpt::node::*;

fn main() {
    let test = TrieNode::LeafNode { encoded_path: vec![65u8, 65u8], value: "test".to_string() };
    print!("{:?}", test)
}