use common::hash::*;
use rlp::RLPSerialize;
use rlp::types::*;

pub type TrieKey = Hash;
pub type EncodedPath = Vec<u8>;

pub const MAX_BRANCHE_NUM: usize = 16usize;
pub const MAX_NIBBLE_VALUE: u8 = 16u8;

const BRANCH_NODE_RLP_SIZE: usize = 16usize;
const BRANCH_NODE_WITH_VALUE_RLP_SIZE: usize = 17usize;
const LEAF_NODE_RLP_SIZE: usize = 2usize;

#[inline]
fn from_slice_to_key(bytes: &Vec<u8>) -> TrieKey {
    let mut a = [0u8; HASH_LEN];
    for i in 0..a.len() {
        // Panics if not enough input
        a[i] = bytes[i];
    }
    a
}

#[inline]
fn from_slice_to_branch(keys: &Vec<TrieKey>) -> [TrieKey; MAX_BRANCHE_NUM] {
    let mut a = [[0u8; HASH_LEN]; MAX_BRANCHE_NUM];
    for i in 0..a.len() {
        // Panics if not enough input
        a[i] = keys[i];
    }
    a
}

#[inline]
pub fn nibble2vec(nibbles: &Vec<u8>) -> Vec<u8> {
    if nibbles.len() % 2 != 0 {
        panic!("Invalid nibble length");
    }
    let mut output: Vec<u8> = vec![];
    let mut i = 0usize;
    loop {
        if i + 2usize > nibbles.len() { break; }
        assert!((nibbles[i] < MAX_NIBBLE_VALUE), "Invalid nibble entry");
        output.append(&mut vec![nibbles[i] * MAX_NIBBLE_VALUE + nibbles[i + 1usize]]);
        i = i + 2usize;
    }
    output
}

#[inline]
pub fn vec2nibble(vec: &Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    for i in 0usize..vec.len() {
        let byte: u8 = vec[i];
        output.append(&mut vec![byte / MAX_NIBBLE_VALUE, byte % MAX_NIBBLE_VALUE]);
    }
    output
}

#[inline]
pub fn encode_path(nibbles: &Vec<u8>, terminated: bool) -> EncodedPath {
    let is_odd = nibbles.len() % 2 != 0;
    if !is_odd && !terminated {
        let mut tmp = vec![0u8, 0u8];
        tmp.append(&mut nibbles.clone());
        nibble2vec(&tmp)
    } else if is_odd && !terminated {
        let mut tmp = vec![1u8];
        tmp.append(&mut nibbles.clone());
        nibble2vec(&tmp)
    } else if !is_odd && terminated {
        let mut tmp = vec![2u8, 0u8];
        tmp.append(&mut nibbles.clone());
        nibble2vec(&tmp)
    } else if is_odd && terminated {
        let mut tmp = vec![3u8];
        tmp.append(&mut nibbles.clone());
        nibble2vec(&tmp)
    } else {
        nibble2vec(&vec![])
    }
}

#[inline]
pub fn decode_path(encoded_path: &Vec<u8>) -> (Vec<u8>, bool) {
    let nibbles = vec2nibble(encoded_path);
    let prefix = nibbles[0];
    match prefix {
        0u8 => {
            (nibbles[2..nibbles.len()].to_vec(), false)
        }
        1u8 => {
            (nibbles[1..nibbles.len()].to_vec(), false)
        }
        2u8 => {
            (nibbles[2..nibbles.len()].to_vec(), true)
        }
        3u8 => {
            (nibbles[1..nibbles.len()].to_vec(), true)
        }
        _ => {
            panic!("Invalid prefix");
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrieNode<T: RLPSerialize + Clone> {
    EMPTY,
    BranchNode { branches: [TrieKey; MAX_BRANCHE_NUM], value: Option<T> },
    ExtensionNode { encoded_path: EncodedPath, key: TrieKey },
    LeafNode { encoded_path: EncodedPath, value: T },
}

impl<T: RLPSerialize + Clone> TrieNode<T> {
    #[inline]
    pub fn new_branch_node(branches: &[TrieKey; MAX_BRANCHE_NUM], value: Option<&T>) -> Self {
        let mut new_branches: [TrieKey; MAX_BRANCHE_NUM] = [zero_hash!(); MAX_BRANCHE_NUM];
        new_branches.copy_from_slice(&branches[0..MAX_BRANCHE_NUM]);
        if let Some(ref_value) = value {
            TrieNode::BranchNode { branches: new_branches, value: Some(ref_value.clone()) }
        } else {
            TrieNode::BranchNode { branches: new_branches, value: None }
        }
    }

    #[inline]
    pub fn new_leaf_node(encoded_path: &EncodedPath, value: &T) -> Self {
        TrieNode::LeafNode { encoded_path: encoded_path.clone(), value: value.clone() }
    }

    #[inline]
    pub fn new_extension_node(encoded_path: &EncodedPath, key: &TrieKey) -> Self {
        TrieNode::ExtensionNode { encoded_path: encoded_path.clone(), key: key.clone() }
    }
}

impl<T: RLPSerialize + Clone> RLPSerialize for TrieNode<T> {
    fn serialize(&self) -> Result<RLP, RLPError> {
        match self {
            &TrieNode::EMPTY => {
                Err(RLPError::RLPEncodingErrorUnencodable)
            }
            &TrieNode::BranchNode { ref branches, ref value } => {
                let mut rlp_list: Vec<RLP> = vec![];
                for elem in branches {
                    let elem_item = RLP::RLPItem(elem.to_vec());
                    rlp_list.append(&mut vec![elem_item]);
                }
                if let &Some(ref r) = value {
                    let mut value_item = r.serialize()?;
                    rlp_list.append(&mut vec![value_item]);
                    Ok(RLP::RLPList(rlp_list))
                } else {
                    Ok(RLP::RLPList(rlp_list))
                }
            }
            &TrieNode::ExtensionNode { ref encoded_path, ref key } => {
                let tmp_path = encoded_path.clone();
                let tmp_key = key.to_vec();
                match (tmp_path, tmp_key) {
                    (l, r) => {
                        Ok(rlp_list![RLP::RLPItem(l), RLP::RLPItem(r)])
                    }
                }
            }
            &TrieNode::LeafNode { ref encoded_path, ref value } => {
                let tmp_path = encoded_path.clone();
                let value_rlp_item = value.serialize();
                match (tmp_path, value_rlp_item) {
                    (l, Ok(r)) => {
                        Ok(rlp_list![RLP::RLPItem(l), r])
                    }
                    _ => Err(RLPError::RLPErrorUnknown("Failed to seriablize leaf node"))
                }
            }
        }
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        match rlp {
            &RLP::RLPList(ref list) => {
                match list.len() {
                    LEAF_NODE_RLP_SIZE => {
                        let path_item = &list[0];
                        let item = &list[1];
                        match (path_item, item) {
                            (&RLP::RLPItem(ref path), rlp) => {
                                let nibbles = vec2nibble(path);
                                // load prefix from the first nibble.
                                let prefix = nibbles[0];
                                if prefix > 1u8 {
                                    //leafnode
                                    Ok(TrieNode::LeafNode {
                                        encoded_path: path.clone(),
                                        value: match T::deserialize(rlp) {
                                            Ok(r) => r,
                                            _ => {
                                                return Err(RLPError::RLPErrorUnknown("Failed to deserialize"));
                                            }
                                        },
                                    })
                                } else {
                                    //extNode
                                    match rlp {
                                        &RLP::RLPItem(ref key) => {
                                            Ok(TrieNode::ExtensionNode {
                                                encoded_path: path.clone(),
                                                key: from_slice_to_key(key),
                                            })
                                        }
                                        _ => { Err(RLPError::RLPErrorUnknown("Failed to deserialize")) }
                                    }
                                }
                            }
                            _ => Err(RLPError::RLPErrorUnknown("Failed to deserialize"))
                        }
                    }
                    //BranchNode
                    BRANCH_NODE_RLP_SIZE => {
                        let mut buffer: Vec<TrieKey> = vec![];
                        let mut index = 0usize;
                        for iter in list {
                            if index == MAX_BRANCHE_NUM { break; }
                            match iter {
                                &RLP::RLPItem(ref value) => {
                                    let key = from_slice_to_key(value);
                                    buffer.append(&mut vec![key]);
                                }
                                _ => {
                                    return Err(RLPError::RLPErrorUnknown("Failed to deserialize"));
                                }
                            }
                            index = index + 1;
                        }
                        Ok(TrieNode::BranchNode { branches: from_slice_to_branch(&buffer), value: None })
                    }
                    BRANCH_NODE_WITH_VALUE_RLP_SIZE => {
                        let mut buffer: Vec<TrieKey> = vec![];
                        let mut index = 0usize;
                        for iter in list {
                            if index == MAX_BRANCHE_NUM { break; }
                            match iter {
                                &RLP::RLPItem(ref value) => {
                                    let key = from_slice_to_key(value);
                                    buffer.append(&mut vec![key]);
                                }
                                _ => { return Err(RLPError::RLPErrorUnknown("Failed to deserialize")); }
                            }
                            index = index + 1;
                        }
                        let value_ref = &list[index];
                        let value = T::deserialize(value_ref)?;
                        Ok(TrieNode::BranchNode { branches: from_slice_to_branch(&buffer), value: Some(value) })
                    }
                    _ => Err(RLPError::RLPErrorUnknown("Failed to deserialize"))
                }
            }
            _ => Err(RLPError::RLPErrorUnknown("Failed to deserialize"))
        }
    }
}

use super::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_nibble2vec() {
        let result = nibble2vec(&vec![
            0x4, 0x8, 0x6, 0x5, 0x6, 0xc, 0x6, 0xc,
            0x6, 0xf, 0x2, 0x0, 0x5, 0x7, 0x6, 0xf,
            0x7, 0x2, 0x6, 0xc, 0x6, 0x4
        ]);
        assert_eq!(String::from_utf8(result).unwrap(), "Hello World".to_string())
    }

    #[test]
    fn test_vec2nibble() {
        let result = vec2nibble(&"Hello World".to_string().into_bytes());
        assert_eq!(vec![
            0x4, 0x8, 0x6, 0x5, 0x6, 0xc, 0x6, 0xc,
            0x6, 0xf, 0x2, 0x0, 0x5, 0x7, 0x6, 0xf,
            0x7, 0x2, 0x6, 0xc, 0x6, 0x4
        ], result)
    }

    #[test]
    fn test_encode_path() {
        let odd_extension = encode_path(&vec![0x1], false);
        let odd_terminated = encode_path(&vec![0x1], true);
        let even_extension = encode_path(&vec![0x1, 0x2], false);
        let even_terminated = encode_path(&vec![0x1, 0x2], true);
        assert_eq!(odd_extension, vec![17]);
        assert_eq!(odd_terminated, vec![49]);
        assert_eq!(even_extension, vec![0, 18]);
        assert_eq!(even_terminated, vec![32, 18]);
    }

    #[test]
    fn test_decode_path() {
        let odd_extension = decode_path(&vec![17]);
        let odd_terminated = decode_path(&vec![49]);
        let even_extension = decode_path(&vec![0, 18]);
        let even_terminated = decode_path(&vec![32, 18]);
        assert_eq!(odd_extension, (vec![0x1], false));
        assert_eq!(odd_terminated, (vec![0x1], true));
        assert_eq!(even_extension, (vec![0x1, 0x2], false));
        assert_eq!(even_terminated, (vec![0x1, 0x2], true));
    }

    #[test]
    fn test_serde_branch() {
        let mut new_branches: [TrieKey; MAX_BRANCHE_NUM] = [zero_hash!(); MAX_BRANCHE_NUM];
        new_branches[0][0] = 0x1;
        new_branches[1][1] = 0x2;
        new_branches[2][2] = 0x3;
        new_branches[3][3] = 0x4;
        let node: TrieNode<String> = TrieNode::new_branch_node(&new_branches, None);
        let rlp = node.serialize().unwrap();
        let target: TrieNode<String> = TrieNode::deserialize(&rlp).unwrap();
        assert_eq!(node, target);
    }

    #[test]
    fn test_serde_extension() {
        let path = vec![1, 2, 3];
        let encoded_path = encode_path(&path, false);
        let node: TrieNode<String> = TrieNode::new_extension_node(&encoded_path, &zero_hash!());
        let rlp = node.serialize().unwrap();
        let target: TrieNode<String> = TrieNode::deserialize(&rlp).unwrap();
        assert_eq!(node, target);
    }

    #[test]
    fn test_serde_leaf() {
        let path = vec![1, 2, 3];
        let encoded_path = encode_path(&path, true);
        let node: TrieNode<String> = TrieNode::new_leaf_node(&encoded_path, &"test".to_string());
        let rlp = node.serialize().unwrap();
        let target: TrieNode<String> = TrieNode::deserialize(&rlp).unwrap();
        assert_eq!(node, target);
    }

    #[test]
    #[should_panic]
    fn test_serde_invalid_leaf() {
        let path = vec![1, 2, 3];
        let encoded_path = encode_path(&path, false);
        let node: TrieNode<String> = TrieNode::new_leaf_node(&encoded_path, &"test".to_string());
        let rlp = node.serialize().unwrap();
        let target: TrieNode<String> = TrieNode::deserialize(&rlp).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_serde_invalid_extension() {
        let path = vec![1, 2, 3];
        let encoded_path = encode_path(&path, true);
        let node: TrieNode<String> = TrieNode::new_extension_node(&encoded_path, &zero_hash!());
        let rlp = node.serialize().unwrap();
        let target: TrieNode<String> = TrieNode::deserialize(&rlp).unwrap();
    }
}