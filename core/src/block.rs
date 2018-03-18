extern crate common;

use self::common::hash::*;
use self::common::address::*;
use self::common::bloom::*;

pub mod nounce {
    /// A BlockNonce is a 64-bit hash which proves (combined with the
    /// mix-hash) that a sufficient amount of computation has been carried
    /// out on a block.
    pub type BlockNounce = [u8; 8];

}


///
///
///
struct BlockHeader {
    parent: Hash,
    uncle: Hash,
    coinbase: Address,
    root: Hash,
    tx_root: Hash,
    receipt_root: Hash,
    bloom: Bloom,
    /*
    Difficulty  *big.Int       `json:"difficulty"       gencodec:"required"`
    Number      *big.Int       `json:"number"           gencodec:"required"`
    GasLimit    uint64         `json:"gasLimit"         gencodec:"required"`
    GasUsed     uint64         `json:"gasUsed"          gencodec:"required"`
    Time        *big.Int       `json:"timestamp"        gencodec:"required"`
    Extra       []byte         `json:"extraData"        gencodec:"required"`
    MixDigest   common.Hash    `json:"mixHash"          gencodec:"required"`
    Nonce       BlockNonce     `json:"nonce"            gencodec:"required"`*/
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_block() {}
}