extern crate common;
extern crate rlp;

use std::iter::Iterator;
use std::fmt;
use self::common::hash::*;
use self::rlp::RLPSerialize;
use self::rlp::types::*;

pub type TrieKey = Hash;
pub type EncodedPath = Vec<u8>;

pub enum TrieNode<T: RLPSerialize> {
    BranchNode { branches: [TrieKey; 16] },
    ExtensionNode { encoded_path: EncodedPath, branch: TrieKey },
    ValueNode { encoded_path: EncodedPath, value: T }
}

impl<T: RLPSerialize> RLPSerialize for TrieNode<T> {
    fn encode(&self) -> Result<EncodedRLP, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }

    fn decode(encoded_rlp: &EncodedRLP) -> Result<Self, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }
}

impl<T: RLPSerialize> fmt::Debug for TrieNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing to see here...")
    }
}