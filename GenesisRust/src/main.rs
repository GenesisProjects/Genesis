extern crate gen_core;
extern crate rlp;
extern crate db;

use gen_core::mpt::trie::*;
use rlp::RLPSerialize;
use db::manager::*;

fn main() {
    let mut test = Trie::new(&SHARED_MANAGER);
    for i in 1 .. 10000 {
        print!("{}\n",i);
        let key = format!("{}", i);
        test.update(&key.as_bytes().to_vec(), &"test".to_string());
    }

    let test_v = test.get(&"124".as_bytes().to_vec());
    print!("{:?}", test_v);
}