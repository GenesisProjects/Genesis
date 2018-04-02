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

}

fn update_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>, v: &T) -> TrieKey {
   match SHARED_MANAGER.lock().unwrap().get(&node.to_vec()) {
       Some(TrieNode::BranchNode::<T> { ref branches, ref value }) => {
            if path.len() == 0 {
                let new_branch_node = TrieNode::new_branch_node(branches, Some(v));
                SHARED_MANAGER.lock().unwrap().put(&new_branch_node)
            } else {
                let nibble = path[0] as usize;
                let next_node = branches[nibble];
                let new_child_key = update_helper(&next_node, &path[1..path.len()].to_vec(), v);
                let mut new_branches = [[0u8; 32]; 16];
                new_branches.copy_from_slice(&branches[0..16]);
                new_branches[nibble] = new_child_key;
                let new_branch_node = TrieNode::new_branch_node(&new_branches, Some(v));
                SHARED_MANAGER.lock().unwrap().put(&new_branch_node)
            }
       },
       Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {

       }

   }
}


