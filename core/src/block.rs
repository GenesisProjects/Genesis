use chrono::{Utc, DateTime};
use common::address::*;
use common::hash::*;
use common::key::Signature;
use rlp::RLPSerialize;
use rlp::types::*;
use mpt::node::TrieKey;


///
///
///
#[derive(Clone)]
pub struct Block {
    num: u64,
    hash: Option<Hash>,
    parent: Hash,
    account_root: TrieKey,
    txs_root: TrieKey,
    signer_addr: Address,
    signature: Option<Signature>,
    date: DateTime<Utc>,
}

impl Block {
    pub fn num(&self) -> u64 {
        self.num
    }

    pub fn account_root(&self) -> TrieKey {
        self.account_root.clone()
    }

    pub fn txs_root(&self) -> TrieKey {
        self.txs_root.clone()
    }
}

impl RLPSerialize for Block {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_block() {}
}