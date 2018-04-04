extern crate gen_core;
extern crate rlp;
extern crate db;

use gen_core::mpt::trie::*;
use rlp::RLPSerialize;
use db::manager::*;

fn main() {
    let mut test = Trie::new([0u8; 32]);
    test.update(&"123".as_bytes().to_vec(), &"test".to_string());
    test.update(&"124".as_bytes().to_vec(), &"test".to_string());
    test.update(&"125".as_bytes().to_vec(), &"test".to_string());
    let test_v = test.get(&"125".as_bytes().to_vec());
    print!("{:?}", test_v);
}