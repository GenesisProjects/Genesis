extern crate gen_core;
extern crate rlp;
extern crate db;

use gen_core::mpt::trie::*;
use rlp::RLPSerialize;
use db::manager::*;

fn main() {
    let mut test = Trie::new([0u8; 32]);
    for i in 1 .. 10000 {
        print!("{}\n",i);
        let key = format!("{}", i);
        test.update(&key.as_bytes().to_vec(), &"test".to_string());
    }

    let test_v = test.get(&"9998".as_bytes().to_vec());
    print!("{:?}", test_v);
}