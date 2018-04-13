use common::hash::*;
use rlp::RLPSerialize;
use rlp::types::*;
use rlp::encoder::*;
use rlp::decoder::*;

pub type TrieKey = Hash;
pub type EncodedPath = Vec<u8>;

pub const MAX_BRANCHE_NUM: usize                = 16usize;
pub const MAX_NIBBLE_VALUE: u8                  = 16u8;

const BRANCH_NODE_RLP_SIZE: usize               = 16usize;
const BRANCH_NODE_WITH_VALUE_RLP_SIZE: usize    = 17usize;
const LEAF_NODE_RLP_SIZE: usize                 = 2usize;

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
    for i in (0usize .. vec.len()) {
        let byte: u8 = vec[i];
        output.append(&mut vec![ byte / MAX_NIBBLE_VALUE, byte % MAX_NIBBLE_VALUE]);
    }
    output
}

#[inline]
pub fn encode_path(nibbles: &Vec<u8>, terminated: bool) -> EncodedPath {
    let is_odd = (nibbles.len() % 2 != 0);
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
            (nibbles[2 .. nibbles.len()].to_vec(), false)
        },
        1u8 => {
            (nibbles[1 .. nibbles.len()].to_vec(), false)
        },
        2u8 => {
            (nibbles[2 .. nibbles.len()].to_vec(), true)
        },
        3u8 => {
            (nibbles[1 .. nibbles.len()].to_vec(), true)
        }
        _ => {
            panic!("Invalid prefix");
        }
    }
}

#[derive(Debug, Clone)]
pub enum TrieNode<T: RLPSerialize + Clone> {
    EMPTY,
    BranchNode { branches: [TrieKey; MAX_BRANCHE_NUM], value: Option<T> },
    ExtensionNode { encoded_path: EncodedPath, key: TrieKey },
    LeafNode { encoded_path: EncodedPath, value: T }
}

impl<T: RLPSerialize + Clone> TrieNode<T> {
    #[inline]
    pub fn new_branch_node(branches: &[TrieKey; MAX_BRANCHE_NUM], value: Option<&T>) -> Self {
        let mut new_branches: [TrieKey; MAX_BRANCHE_NUM] = [zero_hash!(); MAX_BRANCHE_NUM];
        new_branches.copy_from_slice(&branches[0 .. MAX_BRANCHE_NUM]);
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
            },
            &TrieNode::BranchNode{ ref branches, ref value } => {
                let mut rlp_list: Vec<RLP> = vec![];
                for elem in branches {
                    let elem_item = RLP::RLPItem { value: elem.to_vec() };
                    rlp_list.append(&mut vec![elem_item]);
                }
                if let &Some(ref r) = value {
                    let mut value_item = r.serialize()?;
                    rlp_list.append(&mut vec![value_item]);
                    Ok(RLP::RLPList { list: rlp_list })
                } else {
                    Ok(RLP::RLPList { list: rlp_list })
                }
            },
            &TrieNode::ExtensionNode{ ref encoded_path, ref key } => {
                let tmp_path = encoded_path.clone();
                let tmp_key = key.to_vec();
                match (tmp_path, tmp_key) {
                    (l, r) => {
                        let list = vec![
                            RLP::RLPItem { value: l },
                            RLP::RLPItem { value: r }
                        ];
                        Ok(RLP::RLPList { list: list })
                    }
                    _ => {
                        Err(RLPError::RLPErrorUnknown)
                    }
                }
            },
            &TrieNode::LeafNode{ ref encoded_path, ref value } => {
                let tmp_path = encoded_path.clone();
                let value_rlp_item = value.serialize();
                match (tmp_path, value_rlp_item) {
                    (l, Ok(r)) => {
                        let list = vec![RLP::RLPItem { value: l }, r];
                        Ok(RLP::RLPList { list: list })
                    }
                    _ => Err(RLPError::RLPErrorUnknown)
                }
            },
        }
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        match rlp {
           &RLP::RLPList { ref list } => {
               match list.len() {
                   LEAF_NODE_RLP_SIZE => {
                       let path_item = &list[0];
                       let item = &list[1];
                       match (path_item, item)  {
                           (&RLP::RLPItem { value: ref path }, rlp) => {
                               let nibbles = vec2nibble(path);
                               // load prefix from the first nibble.
                               let prefix = nibbles[0];
                               if prefix > 1u8 {
                                   //leafnode
                                   Ok(TrieNode::LeafNode {
                                       encoded_path: path.clone(),
                                       value: match T::deserialize(rlp) {
                                           Ok(r) => r,
                                           _ => { return Err(RLPError::RLPErrorUnknown); }
                                       }
                                   })
                               } else {
                                   //extNode
                                   match rlp {
                                       &RLP::RLPItem { value: ref key } => {
                                           Ok(TrieNode::ExtensionNode {
                                               encoded_path:  path.clone(),
                                               key: from_slice_to_key(key)
                                           })
                                       },
                                        _ => {
                                            Err(RLPError::RLPErrorUnknown)
                                        }
                                   }
                               }
                           },
                           _ => Err(RLPError::RLPErrorUnknown)
                       }
                   },
                   //BranchNode
                   BRANCH_NODE_RLP_SIZE => {
                       let mut buffer: Vec<TrieKey> = vec![];
                       let mut index= 0usize;
                       for iter in list {
                           if index == MAX_BRANCHE_NUM { break; }
                           match iter {
                               &RLP::RLPItem { ref value } => {
                                   let key = from_slice_to_key(value);
                                   buffer.append(&mut vec![key]);
                               },
                               _ => { return Err(RLPError::RLPErrorUnknown); }
                           }
                           index = index + 1;
                       }
                       Ok(TrieNode::BranchNode { branches: from_slice_to_branch(&buffer), value: None })
                   },
                   BRANCH_NODE_WITH_VALUE_RLP_SIZE => {
                       let mut buffer: Vec<TrieKey> = vec![];
                       let mut index= 0usize;
                       for iter in list {
                           if index == MAX_BRANCHE_NUM { break; }
                           match iter {
                               &RLP::RLPItem { ref value } => {
                                   let key = from_slice_to_key(value);
                                   buffer.append(&mut vec![key]);
                               },
                               _ => { return Err(RLPError::RLPErrorUnknown); }
                           }
                           index = index + 1;
                       }
                       let value_ref = &list[index];
                       let value = T::deserialize(value_ref)?;
                       Ok(TrieNode::BranchNode { branches: from_slice_to_branch(&buffer), value: Some(value) })
                   },
                   _ => Err(RLPError::RLPErrorUnknown)
               }
           },
           &RLP::RLPItem { ref value } => Err(RLPError::RLPErrorUnknown)
       }
    }
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_node() {}
}