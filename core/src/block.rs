use common::hash::*;
use common::address::*;
use common::bloom::*;
use num::bigint::BigInt;
use rlp::RLPSerialize;
use rlp::types::*;

use log::Log;

pub mod nounce {
    /// A BlockNonce is a 64-bit hash which proves (combined with the
    /// mix-hash) that a sufficient amount of computation has been carried
    /// out on a block.
    pub type BlockNounce = [u8; 8];

}

///
///
///
#[derive(Clone, Debug)]
pub struct Block {
    pub parent: Hash,
    pub uncle: Hash,
    pub coinbase: Address,
    pub root: Hash,
    pub tx_root: Hash,
    pub receipt_root: Hash,
    //pub logs_bloom: Bloom<Log>,
    pub difficulty: BigInt,
    pub number: BigInt,
    pub gas_used: u64,
    pub time: BigInt,
    pub extra: Vec<u8>,
    pub digest: Hash,
    pub nounce: nounce::BlockNounce
}

impl RLPSerialize for Block {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_block() {}
}