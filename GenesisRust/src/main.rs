extern crate db;
extern crate gen_core;

use gen_core::mpt::trie::*;
use db::manager::SHARED_MANAGER;

fn main() {
    let mut tree: Trie<String> = Trie::new(&SHARED_MANAGER);
    tree.update(&"hello".to_string().into_bytes(), &"test".to_string());
    tree.update(&"hello".to_string().into_bytes(), &"test".to_string());
}