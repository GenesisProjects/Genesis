extern crate common;
extern crate rlp;

use std::iter::Iterator;
use std::fmt;
use self::common::hash::*;

use self::rlp::RLPSerialize;
use self::rlp::types::*;
use self::rlp::encoder::*;
use self::rlp::decoder::*;

pub type TrieKey = Hash;
pub type EncodedPath = Vec<u8>;

fn encode_path(path: &Vec<u8>) -> EncodedPath {
    path.as_bytes()
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