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
    let test = TrieNode::ExtensionNode::<String> { encoded_path: vec![00u8, 65u8], key: [0u8; 32] };
    let rlp: RLP = match test.serialize() {
        Ok(r) => r,
        Err(_) => panic!("here")
    };

    let rlp_encoded = SHARED_ENCODER.lock().unwrap().encode(&rlp);
    let rlp_decoded = Decoder::decode(&rlp_encoded);
    print!("rlp{:?}\n", rlp);
    print!("r{:?}\n", rlp_encoded);
    print!("d{:?}\n", rlp_decoded);
    SHARED_MANAGER.lock().unwrap().put(&test);
}