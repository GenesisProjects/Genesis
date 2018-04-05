extern crate common;
extern crate db;
extern crate rlp;

use super::node::*;
use self::db::manager::*;
use self::rlp::RLPSerialize;
use self::common::hash::*;

use std::cmp::min;
use std::marker::PhantomData;
use std::sync::Mutex;

pub struct Trie<'a, T: RLPSerialize + Clone> {
    root: TrieKey,
    db: &'a Mutex<DBManager>,
    phantom: PhantomData<T>
}

impl<'a, T> Trie<'a, T> where T: RLPSerialize + Clone {
    pub fn get(&self, path: &Vec<u8>) -> Option<T> {
        get_helper(&self.root, &vec2nibble(path), self.db)
    }

    pub fn delete(&mut self, path: &Vec<u8>) {
        self.root = delete_helper::<T>(&self.root, &vec2nibble(path), self.db);
    }

    pub fn update(&mut self, path: &Vec<u8>, v: &T) {
        self.root = update_helper(&self.root, &vec2nibble(path), v, self.db);
    }

    pub fn new(db: &'a Mutex<DBManager>) -> Trie<'a, T> {
        Trie::<'a, T>{ root: zero_hash!(), db: db, phantom: PhantomData }
    }
}

const PATH_MAX_LEN: usize = 64usize;


macro_rules! delete {
    ($node:expr, $db:expr) => {{
        $db.lock().unwrap().delete(&($node).to_vec());
    }};
}

macro_rules! update {
    ($node:expr, $db:expr) => {
        $db.lock().unwrap().put($node)
    };
}

macro_rules! fetch {
    ($node:expr, $db:expr) => {
        $db.lock().unwrap().get(&($node).to_vec())
    };
}

#[inline]
fn cmp_path(path1: &Vec<u8>, path2: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let min_size = min(path1.len(), path2.len());
    let mut diff_pos = 0usize;
    for i in 0..min_size {
        if path1[i] != path2[i] {
            break;
        }
        diff_pos = i + 1;
    }
    (
        path1[0..diff_pos].to_vec(),
        path1[diff_pos..path1.len()].to_vec(),
        path2[diff_pos..path2.len()].to_vec()
    )
}

fn get_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>, db: &Mutex<DBManager>) -> Option<T> {
    let node_type = fetch!(node, db);
    match node_type {
        Some(TrieNode::BranchNode::<T> { ref branches, ref value }) => {
            if let &Some(ref value) = value {
                if path.len() == 0 {
                    Some(value.clone())
                } else {
                    let nibble = path[0] as usize;
                    assert!((nibble as u8) < MAX_NIBBLE_VALUE, "Invalid nibble");
                    let next_node = branches[nibble];
                    get_helper(&next_node, &path[1..path.len()].to_vec(), db)
                }
            } else {
                if path.len() == 0 {
                    None
                } else {
                    let nibble = path[0] as usize;
                    assert!((nibble as u8) < MAX_NIBBLE_VALUE, "Invalid nibble");
                    let next_node = branches[nibble];
                    get_helper(&next_node, &path[1..path.len()].to_vec(), db)
                }
            }
        },
        Some(TrieNode::ExtensionNode { ref encoded_path, ref key }) => {
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(encoded_path);
            let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
            get_helper(key, &remain_path, db)
        },
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(encoded_path);
            let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
            if remain_cur_path.len() == 0 && remain_path.len() == 0 {
                Some(value.clone())
            } else { None }
        },
        None => { None },
        _ => panic!("Unknown error!")
    }
}

fn delete_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>, db: &Mutex<DBManager>) -> TrieKey {
    let node_type = fetch!(node, db);
    match node_type {
        Some(TrieNode::BranchNode::<T> { ref branches, ref value }) => {
            let mut new_branches: [TrieKey; MAX_BRANCHE_NUM] = [zero_hash!(); MAX_BRANCHE_NUM];
            for i in 0 .. MAX_BRANCHE_NUM {
                new_branches[i] = branches[i];
            }
            if path.len() == 0 {
                let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, None);
                delete!(node, db);
                update!(new_branch_node, db)
            } else {
                let nibble = path[0] as usize;
                assert!((nibble as u8) < MAX_NIBBLE_VALUE, "Invalid nibble");
                let next_node = &branches[nibble];
                let new_node = delete_helper::<T>(next_node, &path[1 .. path.len()].to_vec(), db);
                new_branches[nibble] = new_node;
                let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, value.as_ref());
                delete!(node, db);
                update!(new_branch_node, db)
            }
        },
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(encoded_path);
            let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
            if remain_cur_path.len() == 0 && remain_path.len() == 0 {
                delete!(node, db);
                zero_hash!()
            } else {
                let mut ret_node: TrieKey = zero_hash!();
                ret_node.copy_from_slice(&node[0 .. HASH_LEN]);
                ret_node
            }
        },
        Some(TrieNode::ExtensionNode::<T> { ref encoded_path, ref key }) => {
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(encoded_path);
            let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
            if remain_cur_path.len() != 0 {
                let mut ret_node: TrieKey = zero_hash!();
                ret_node.copy_from_slice(&node[0 .. HASH_LEN]);
                ret_node
            } else {
                let new_child_node = delete_helper::<T>(key, &remain_path, db);
                let new_extension_node = &TrieNode::<T>::new_extension_node(encoded_path, &new_child_node);
                delete!(node, db);
                update!(new_extension_node, db)
            }
        },
        None => {
            zero_hash!()
        },
        _ => panic!("Unknown error!")
    }
}

fn update_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>, v: &T, db: &Mutex<DBManager>) -> TrieKey {
    let node_type = fetch!(node, db);
    match node_type {
        Some(TrieNode::BranchNode::<T> { ref branches, ref value }) => {
            if path.len() == 0 {
                let new_branch_node = &TrieNode::new_branch_node(branches, Some(v));
                delete!(node, db);
                update!(new_branch_node, db)
            } else {
                let nibble = path[0] as usize;
                assert!((nibble as u8) < MAX_NIBBLE_VALUE, "Invalid nibble");
                let next_node = branches[nibble];
                let new_child_key = update_helper(&next_node, &path[1..path.len()].to_vec(), v, db);
                let mut new_branches = [zero_hash!(); MAX_BRANCHE_NUM];
                new_branches.copy_from_slice(&branches[0 .. MAX_BRANCHE_NUM]);
                new_branches[nibble] = new_child_key;
                let new_branch_node = &TrieNode::new_branch_node(&new_branches, value.as_ref());
                delete!(node, db);
                update!(new_branch_node, db)
            }
        },
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            update_kv_node_helper(node, path, v, db)
        },
        Some(TrieNode::ExtensionNode::<T> { ref encoded_path, ref key }) => {
            update_kv_node_helper(node, path, v, db)
        },
        None => {
            let encoded_path = encode_path(&path, true);
            let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, v);
            delete!(node, db);
            update!(new_leaf_node, db)
        },
        _ => { panic!("Unknown error!") }
   }
}

fn update_kv_node_helper<T: RLPSerialize + Clone>(node: &TrieKey, path: &Vec<u8>, new_value: &T, db: &Mutex<DBManager>) -> TrieKey {
    let node_type = fetch!(node, db);
    match node_type {
        // if the node is a leaf node
        Some(TrieNode::LeafNode::<T> { ref encoded_path, ref value }) => {
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(encoded_path);
            if !terminated {
                panic!("Malformed path")
            } else {
                // compute the shared path of the node path and the input path, then split remain paths
                let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
                // compute new nodes for remain paths, attach them to a new branch node
                let branch_key = if remain_path.len() == remain_cur_path.len() && remain_path.len() == 0 {
                    let new_leaf_node = &TrieNode::new_leaf_node(path, new_value);
                    update!(new_leaf_node, db)
                } else if remain_cur_path.len() == 0 {
                    let mut new_branches = [zero_hash!(); MAX_BRANCHE_NUM];
                    let encoded_path = encode_path(&remain_path[1 .. remain_path.len()].to_vec(), true);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, new_value);
                    let child_key = update!(new_leaf_node, db);
                    new_branches[remain_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::new_branch_node(&new_branches, Some(value));
                    update!(new_branch_node, db)
                } else if remain_path.len() == 0 {
                    let mut new_branches = [zero_hash!(); MAX_BRANCHE_NUM];
                    let encoded_path = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), true);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, value);
                    let child_key = update!(new_leaf_node, db);
                    new_branches[remain_cur_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::new_branch_node(&new_branches, Some(new_value));
                    update!(new_branch_node, db)
                } else {
                    let mut new_branches = [zero_hash!(); MAX_BRANCHE_NUM];
                    let encoded_path_cur = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), true);
                    let encoded_path = encode_path(&remain_path[1 .. remain_path.len()].to_vec(), true);
                    let new_leaf_node_cur = &TrieNode::new_leaf_node(&encoded_path_cur, value);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, new_value);
                    let child_key_cur = update!(new_leaf_node_cur, db);
                    let child_key = update!(new_leaf_node, db);
                    new_branches[remain_cur_path[0] as usize] = child_key_cur;
                    new_branches[remain_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, None);
                    update!(new_branch_node, db)
                };
                // delete current node
                delete!(node, db);
                // if the share path is empty, then return the branch node, else make a new extension node point to the branch node.
                if shared_path.len() == 0 { branch_key } else {
                    let encoded_path = encode_path(&shared_path, false);
                    let new_extension_node = &TrieNode::<T>::new_extension_node(&encoded_path, &branch_key);
                    update!(new_extension_node, db)
                }
            }
        },
        // if the node is a extension node
        Some(TrieNode::ExtensionNode::<T> { ref encoded_path, ref key }) => {
            // decode the path for the node
            let (ref cur_path, terminated) = decode_path(encoded_path);
            if terminated { panic!("Malformed path") } else {
                // compute the shared path of the node path and the input path, then split remain paths
                let (shared_path, remain_cur_path, remain_path) = cmp_path(cur_path, path);
                // compute new nodes for remain paths, attach them to a new branch node
                let branch_key = if remain_path.len() == remain_cur_path.len() && remain_path.len() == 0 {
                    update_helper(key, &remain_path, new_value, db)
                } else if remain_cur_path.len() == 0 {
                    update_helper(key, &remain_path, new_value, db)
                } else if remain_path.len() == 0 {
                    let mut new_branches = [zero_hash!(); MAX_BRANCHE_NUM];
                    if remain_cur_path.len() == 1 {
                        new_branches[remain_cur_path[0] as usize] = *key;
                    } else {
                        let encoded_path = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), false);
                        let new_extension_node = &TrieNode::<T>::new_extension_node(&encoded_path, key);
                        let child_key = update!(new_extension_node, db);
                        new_branches[remain_cur_path[0] as usize] = child_key;
                    }
                    let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, Some(new_value));
                    update!(new_branch_node, db)
                } else {
                    let mut new_branches = [zero_hash!(); MAX_BRANCHE_NUM];
                    let encoded_path_cur = encode_path(&remain_cur_path[1 .. remain_cur_path.len()].to_vec(), false);
                    let encoded_path = encode_path(&remain_path[1 .. remain_path.len()].to_vec(), true);
                    let new_extension_node_cur = &TrieNode::<T>::new_extension_node(&encoded_path_cur, key);
                    let new_leaf_node = &TrieNode::new_leaf_node(&encoded_path, new_value);
                    let child_key_cur = update!(new_extension_node_cur, db);
                    let child_key = update!(new_leaf_node, db);
                    new_branches[remain_cur_path[0] as usize] = child_key_cur;
                    new_branches[remain_path[0] as usize] = child_key;
                    let new_branch_node = &TrieNode::<T>::new_branch_node(&new_branches, None);
                    update!(new_branch_node, db)
                };
                delete!(node, db);
                // if the share path is empty, then return the branch node, else make a new extension node point to the branch node.
                if shared_path.len() == 0 { branch_key } else {
                    let encoded_path = encode_path(&shared_path, false);
                    let new_extension_node = &TrieNode::<T>::new_extension_node(&encoded_path, &branch_key);
                    update!(new_extension_node, db)
                }
            }
        },
        _ => panic!("Input node is not a kv node.")
    }
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_trie() {}
}