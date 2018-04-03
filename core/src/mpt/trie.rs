extern crate common;
extern crate db;
extern crate rlp;

use super::node::*;
use self::db::manager::*;
use self::rlp::RLPSerialize;

use std::cmp::min;
use std::marker::PhantomData;

const PATH_MAX_LEN: usize = 32usize;

macro_rules! delete {
    ($node:expr) => {{
        SHARED_MANAGER.lock().unwrap().delete(&($node).to_vec());
    }};
}

macro_rules! update {
    ($node:expr) => {
        SHARED_MANAGER.lock().unwrap().put($node)
    };
}

macro_rules! fetch {
    ($node:expr) => {
        SHARED_MANAGER.lock().unwrap().get(&($node).to_vec())
    };
}

struct Trie<T: RLPSerialize + Clone> {
    root: TrieKey,
    phantom: PhantomData<T>
}

impl<T> Trie<T> where T: RLPSerialize + Clone {
    pub fn update(&mut self, path: &Vec<u8>, v: &T) {
        self.root = update_helper(&self.root, path, v);
    }

    pub fn get(&self, path: &Vec<u8>) -> Option<T> {
        get_helper(&self.root, path)
    }
}

fn get_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>) -> Option<T> {
    match fetch!(node) {
        Some(TrieNode::BranchNode::<T> { ref branches, ref value }) => {
            if let &Some(ref value) = value {
                if path.len() == 0 {
                    Some(value.clone())
                } else {
                    let nibble = path[0] as usize;
                    let next_node = branches[nibble];
                    get_helper(&next_node, &path[1..path.len()].to_vec())
                }
            } else {
                if path.len() == 0 {
                    None
                } else {
                    let nibble = path[0] as usize;
                    let next_node = branches[nibble];
                    get_helper(&next_node, &path[1..path.len()].to_vec())
                }
            }
        },
        Some(TrieNode::ExtensionNode { ref encoded_path, ref key }) => {
            let nibbles = vec2nibble(encoded_path);
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(&nibbles);
            let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
            get_helper(key, &remain_path)
        },
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            let nibbles = vec2nibble(encoded_path);
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(&nibbles);
            let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
            if remain_cur_path.len() == 0 && remain_path.len() == 0 {
                Some(value.clone())
            } else {
                None
            }
        },
        _ => panic!("Unknown error!")
    }
}

fn update_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>, v: &T) -> TrieKey {
    match fetch!(node) {
        Some(TrieNode::BranchNode::<T> { ref branches, ref value }) => {
            if path.len() == 0 {
                let new_branch_node = &TrieNode::new_branch_node(branches, Some(v));
                delete!(node);
                update!(new_branch_node)
            } else {
                let nibble = path[0] as usize;
                let next_node = branches[nibble];
                let new_child_key = update_helper(&next_node, &path[1..path.len()].to_vec(), v);
                let mut new_branches = [[0u8; 32]; 16];
                new_branches.copy_from_slice(&branches[0..16]);
                new_branches[nibble] = new_child_key;
                let new_branch_node = &TrieNode::new_branch_node(&new_branches, Some(v));
                delete!(node);
                update!(new_branch_node)
            }
        },
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            update_kv_node_helper(node, encoded_path, v)
        },
        Some(TrieNode::ExtensionNode::<T> { ref encoded_path, ref key }) => {
            update_kv_node_helper(node, encoded_path, v)
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

fn update_kv_node_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>, new_value: &T) -> TrieKey {
    match fetch!(node) {
        // if the node is a leaf node
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            let nibbles = vec2nibble(encoded_path);
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(&nibbles);
            if !terminated {
                panic!("Malformed path")
            } else {
                // compute the shared path of the node path and the input path, then split remain paths
                let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
                // compute new nodes for remain paths, attach them to a new branch node
                let branch_key = if remain_path.len() == remain_cur_path.len() && remain_path.len() == 0 {
                    let new_leaf_node = &TrieNode::new_leaf_node(path, new_value);
                    update!(new_leaf_node)
                } else if remain_cur_path.len() == 0 {
                    let mut new_branches = [[0u8; 32]; 16];
                    let encoded_path = encode_path(&vec2nibble(&remain_path[1 .. remain_path.len()].to_vec()), true);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, new_value);
                    let child_key = update!(new_leaf_node);
                    new_branches[remain_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::new_branch_node(&new_branches, Some(value));
                    update!(new_branch_node)
                } else if remain_path.len() == 0 {
                    let mut new_branches = [[0u8; 32]; 16];
                    let encoded_path = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), true);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, value);
                    let child_key = update!(new_leaf_node);
                    new_branches[remain_cur_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::new_branch_node(&new_branches, Some(new_value));
                    update!(new_branch_node)
                } else {
                    let mut new_branches = [[0u8; 32]; 16];
                    let encoded_path_cur = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), true);
                    let encoded_path = encode_path(&remain_path[1 .. remain_path.len()].to_vec(), true);
                    let new_leaf_node_cur = &TrieNode::new_leaf_node(&encoded_path_cur, value);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, new_value);
                    let child_key_cur = update!(new_leaf_node_cur);
                    let child_key = update!(new_leaf_node);
                    new_branches[remain_cur_path[0] as usize] = child_key_cur;
                    new_branches[remain_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, None);
                    update!(new_branch_node)
                };

                // delete current node
                delete!(node);
                // if the share path is empty, then return the branch node, else make a new extension node point to the branch node.
                if shared_path.len() == 0 {
                    branch_key
                } else {
                    let encoded_path = encode_path(&shared_path, false);
                    let new_extension_node = TrieNode::<T>::new_extension_node(&encoded_path, &branch_key);
                    SHARED_MANAGER.lock().unwrap().put(&new_extension_node)
                }
            }
        },
        // if the node is a extension node
        Some(TrieNode::ExtensionNode::<T> { ref encoded_path, ref key }) => {
            let nibbles = vec2nibble(encoded_path);
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(&nibbles);
            if terminated {
                panic!("Malformed path")
            } else {
                // compute the shared path of the node path and the input path, then split remain paths
                let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
                // compute new nodes for remain paths, attach them to a new branch node
                let branch_key = if remain_path.len() == remain_cur_path.len() && remain_path.len() == 0 {
                    update_helper(key, &remain_path, new_value)
                } else if remain_cur_path.len() == 0 {
                    update_helper(key, &remain_path, new_value)
                } else if remain_path.len() == 0 {
                    let mut new_branches = [[0u8; 32]; 16];
                    if remain_cur_path.len() == 1 {
                        new_branches[remain_cur_path[0] as usize] = *key;
                    } else {
                        let encoded_path = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), false);
                        let new_extension_node = &TrieNode::<T>::new_extension_node(&encoded_path, key);
                        let child_key = update!(new_extension_node);
                        new_branches[remain_cur_path[0] as usize] = child_key;
                    }
                    let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, Some(new_value));
                    update!(new_branch_node)
                } else {
                    let mut new_branches = [[0u8; 32]; 16];
                    let encoded_path_cur = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), false);
                    let encoded_path = encode_path(&remain_path[1 .. remain_path.len()].to_vec(), true);
                    let new_extension_node_cur = &TrieNode::<T>::new_extension_node(&encoded_path_cur, key);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, new_value);
                    let child_key_cur = update!(new_extension_node_cur);
                    let child_key = update!(new_leaf_node);
                    new_branches[remain_cur_path[0] as usize] = child_key_cur;
                    new_branches[remain_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, None);
                    update!(new_branch_node)
                };
                delete!(node);
                // if the share path is empty, then return the branch node, else make a new extension node point to the branch node.
                if shared_path.len() == 0 {
                    branch_key
                } else {
                    let encoded_path = encode_path(&shared_path, false);
                    let new_extension_node = TrieNode::<T>::new_extension_node(&encoded_path, &branch_key);
                    SHARED_MANAGER.lock().unwrap().put(&new_extension_node)
                }
            }
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
    (
        path1[0 .. diff_pos].to_vec(),
        path1[diff_pos .. path1.len()].to_vec(),
        path2[diff_pos .. path2.len()].to_vec()
    )
}



