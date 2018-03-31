extern crate common;
extern crate gen_core;
extern crate rlp;
extern crate db;

use self::rlp::encoder::{ Encoder, SHARED_ENCODER };
use self::rlp::decoder::Decoder;
use self::rlp::types::RLP;
use self::rlp::RLPSerialize;

use self::db::manager::*;

use self::gen_core::mpt::node::*;
use self::common::hash::SerializableAndSHA256Hashable;

fn main() {
    let mut i: u64 = 0;
    loop {
        if i >= 9999999999999999 {break;}
        let test = TrieNode::ExtensionNode::<String> { encoded_path: vec![05u8, 65u8], key: [0u8; 32] };
        SHARED_MANAGER.lock().unwrap().put(&test);
        i = i + 1;
    }
}