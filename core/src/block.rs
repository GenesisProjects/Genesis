extern crate common;
extern crate num;
extern crate rlp;

use self::common::hash::*;
use self::common::address::*;
use self::common::bloom::*;
use self::num::bigint::BigInt;
use self::rlp::RLPSerialize;
use self::rlp::types::*;

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
#[derive(Debug)]
pub struct Block {
    pub parent: Hash,
    pub uncle: Hash,
    pub coinbase: Address,
    pub root: Hash,
    pub tx_root: Hash,
    pub receipt_root: Hash,
    pub logs_bloom: Bloom<Log>,
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