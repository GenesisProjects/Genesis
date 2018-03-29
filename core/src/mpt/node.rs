extern crate common;
extern crate rlp;

use std::fmt;
use std::iter::Iterator;
use self::common::hash::*;

use self::rlp::RLPSerialize;
use self::rlp::types::*;
use self::rlp::encoder::*;
use self::rlp::decoder::*;

pub type TrieKey = Hash;
pub type EncodedPath = Vec<u8>;

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
        output.append(&mut vec![nibble[i] * 16u8 + nibble[i + 1]]);
        i = i + 2usize;
    }
    output
}

pub fn vec2nibble(path: &Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    for i in (0usize .. path.len()) {
        let byte: u8 = path[i];
        output.append(&mut vec![ byte / 16u8, byte % 16u8]);
    }
    output
}

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
    fn encode(&self) -> Result<EncodedRLP, RLPError> {
        match self {
            &TrieNode::EMPTY => {
                Err(RLPError::RLPErrorUnknown)
            },
            &TrieNode::BranchNode{ ref branches, ref value } => {
                Err(RLPError::RLPErrorUnknown)
            },
            &TrieNode::ExtensionNode{ ref encoded_path, ref key } => {
                Err(RLPError::RLPErrorUnknown)
            },
            &TrieNode::LeafNode{ ref encoded_path, ref value } => {
                Err(RLPError::RLPErrorUnknown)
            },
        }
    }

    fn decode(encoded_rlp: &EncodedRLP) -> Result<Self, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }
}