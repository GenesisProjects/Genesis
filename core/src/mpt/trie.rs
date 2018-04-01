/*
type Trie struct {
    db           *Database
    root         node
    originalRoot common.Hash

    // Cache generation values.
    // cachegen increases by one with each commit operation.
    // new nodes are tagged with the current generation and unloaded
    // when their generation is older than than cachegen-cachelimit.
    cachegen, cachelimit uint16
}*/
extern crate common;
extern crate db;
extern crate rlp;

use super::node::*;
use self::db::manager::*;
use self::rlp::RLPSerialize;

struct Trie<T: RLPSerialize + Clone> {
    root: TrieNode<T>
}

impl<T> Trie<T> where T: RLPSerialize + Clone {
    fn update_helper(node: &TrieNode<T>, nibbles: Vec<u8>) {
        if nibbles.len() == 0 {
            let mut cur_node = if let Some(cur_node) = SHARED_MANAGER
                .lock()
                .unwrap()
                .get_node(node) {
                cur_node
            } else {
                match node {
                    &TrieNode::BranchNode { ref branches, ref value } => {
                        TrieNode::new_branch_node(value)
                    },
                    &TrieNode::LeafNode { ref path, ref value } => {
                        TrieNode::new_branch_node(value)
                    },
                }
            };
            cur_node
        }
    }
}


