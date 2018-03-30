extern crate common;
extern crate rlp;

use std::fmt;
use std::iter::Iterator;
use self::common::hash::*;

use self::rlp::RLPSerialize;
use self::rlp::types::*;
use self::rlp::encoder::*;
use self::rlp::decoder::*;

pub type TrieKey = [u8; 32];
pub type EncodedPath = Vec<u8>;


#[inline]
pub fn nibble2vec(nibble: &Vec<u8>) -> Vec<u8> {
    if nibble.len() % 2 != 0 {
        panic!("Invalid nibble length");
    }
    let mut output: Vec<u8> = vec![];
    let mut i = 0usize;
    loop {
        if nibble[i] >= 16u8 {
            panic!("Invalid nibble entry");
        }
        if i >= nibble.len() / 2usize { break; }
        output.append(&mut vec![nibble[i] * 16u8 + nibble[i + 1usize]]);
        i = i + 2usize;
    }
    output
}

#[inline]
pub fn vec2nibble(path: &Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    for i in (0usize .. path.len()) {
        let byte: u8 = path[i];
        output.append(&mut vec![ byte / 16u8, byte % 16u8]);
    }
    output
}

#[inline]
pub fn encode_path(nibble: &Vec<u8>, terminated: bool) -> EncodedPath {
    let is_odd = (nibble.len() % 2 != 0);
    if !is_odd && !terminated {
        let mut tmp = vec![0u8, 0u8];
        tmp.append(&mut nibble.clone());
        nibble2vec(&tmp)
    } else if is_odd && !terminated {
        let mut tmp = vec![1u8];
        tmp.append(&mut nibble.clone());
        nibble2vec(&tmp)
    } else if !is_odd && terminated {
        let mut tmp = vec![2u8, 0u8];
        tmp.append(&mut nibble.clone());
        nibble2vec(&tmp)
    } else if is_odd && terminated {
        let mut tmp = vec![3u8];
        tmp.append(&mut nibble.clone());
        nibble2vec(&tmp)
    } else {
        nibble2vec(&vec![])
    }
}

#[inline]
pub fn decode_path(encoded_path: &Vec<u8>) -> (Vec<u8>, bool) {
    let prefix = encoded_path[0] / 16u8;
    match prefix {
        0u8 => {
            let nibble = vec2nibble(encoded_path);
            (nibble[2 .. nibble.len()].to_vec(), false)
        },
        1u8 => {
            let nibble = vec2nibble(encoded_path);
            (nibble[1 .. nibble.len()].to_vec(), false)
        },
        2u8 => {
            let nibble = vec2nibble(encoded_path);
            (nibble[2 .. nibble.len()].to_vec(), true)
        },
        3u8 => {
            let nibble = vec2nibble(encoded_path);
            (nibble[1 .. nibble.len()].to_vec(), true)
        }
        _ => {
            panic!("Invalid prefix");
        }
    }
}

#[derive(Debug, Clone)]
pub enum TrieNode<T: RLPSerialize> {
    EMPTY,
    BranchNode { branches: [TrieKey; 16], value: T },
    ExtensionNode { encoded_path: EncodedPath, key: TrieKey },
    LeafNode { encoded_path: EncodedPath, value: T }
}

impl<T: RLPSerialize> RLPSerialize for TrieNode<T> {
    fn serialize(&self) -> Result<RLP, RLPError> {
        match self {
            &TrieNode::EMPTY => {
                Err(RLPError::RLPEncodingErrorUnencodable)
            },
            &TrieNode::BranchNode{ ref branches, ref value } => {
                let mut value_item = value.serialize()?;
                let mut rlp_list: Vec<RLP> = vec![];
                for elem in branches {
                    let elem_str_r = String::from_utf8((elem as &TrieKey).to_vec());
                    match elem_str_r {
                        Ok(r) => {
                            let elem_item = RLP::RLPItem { value: r };
                            rlp_list.append(&mut vec![elem_item]);
                        },
                        Err(e) => {
                            return Err(RLPError::RLPErrorUTF8);
                        }
                    }
                }
                Ok(RLP::RLPList { list: rlp_list })
            },
            &TrieNode::ExtensionNode{ ref encoded_path, ref key } => {
                let path_str_r = String::from_utf8(encoded_path.to_vec());
                let key_str_r = String::from_utf8(key.to_vec());
                match (path_str_r, key_str_r) {
                    (Ok(l), Ok(r)) => {
                        let list = vec![RLP::RLPItem { value: l }, RLP::RLPItem { value: r }];
                        Ok(RLP::RLPList { list: list })
                    }
                    _ => {
                        Err(RLPError::RLPErrorUTF8)
                    }
                }
            },
            &TrieNode::LeafNode{ ref encoded_path, ref value } => {
                let path_str_r = String::from_utf8(encoded_path.to_vec());
                let value_rlp_item = value.serialize();
                match (path_str_r, value_rlp_item) {
                    (Ok(l), Ok(r)) => {
                        let list = vec![RLP::RLPItem { value: l }, r];
                        Ok(RLP::RLPList { list: list })
                    }
                    (Err(l), Ok(r)) => {
                        Err(RLPError::RLPErrorUTF8)
                    }
                    (Ok(l), Err(r)) => {
                        Err(RLPError::RLPErrorUnknown)
                    }
                    _ => {
                        Err(RLPError::RLPErrorUnknown)
                    }
                }
            },
        }
    }

    fn deserialize(encoded_rlp: &RLP) -> Result<Self, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }
}