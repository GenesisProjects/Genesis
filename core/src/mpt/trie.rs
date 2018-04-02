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

use std::cmp::min;

const PATH_MAX_LEN: usize = 32usize;

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
            panic!("TODO")
        },
        Some(TrieNode::ExtensionNode::<T> { ref encoded_path, ref key }) => {
            panic!("TODO")
        },
        None => {
            let encoded_path = encode_path(&vec2nibble(path), true);
            let new_leaf_node = TrieNode::new_leaf_node(&encoded_path, v);
            SHARED_MANAGER.lock().unwrap().put(&new_leaf_node)
        },
        _ => {
            panic!("Unknown error!")
        }
   }
}

fn update_kev_node<T: RLPSerialize + Clone>(node: &TrieKey, cur_path: &Vec<u8>, v: &T) -> TrieKey {
    match SHARED_MANAGER.lock().unwrap().get(&node.to_vec()) {
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            let nibbles = vec2nibble(encoded_path);
            let (path, terminated) = decode_path(&nibbles);
            if !terminated {
                panic!("Malformed path")
            } else {
                panic!("TODO")
            }
        },
        Some(TrieNode::ExtensionNode::<T> { ref encoded_path, ref key }) => {
            panic!("TODO")
        },
        _ => panic!("Input node is not a kv node.")
    }
}

#[inline]
fn cmp_path(path1: &Vec<u8>, path2: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let min_size = min(path1.len(), path2.len());
    let mut diff_pos = 0usize;
    for i in 0 .. min_size {
        if path1[i] != path2[i] {
            diff_pos = i;
            break;
        }
    }
    (path1[0 .. diff_pos].to_vec(), path1[diff_pos .. path1.len()].to_vec(), path2[diff_pos .. path2.len()].to_vec())
}



