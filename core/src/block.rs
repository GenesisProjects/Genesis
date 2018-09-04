use common::hash::*;
use common::address::*;
use num::bigint::BigInt;
use rlp::RLPSerialize;
use rlp::types::*;
use mpt::node::TrieKey;


///
///
///
#[derive(Clone, Debug)]
pub struct Block {
    num: u64,
    parent: Hash,
    uncle: Hash,
    coinbase: Address,
    account_root: TrieKey,
    txs_root: TrieKey,
    time: BigInt,
    extra: Vec<u8>,
    digest: Hash
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