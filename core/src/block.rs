
pub mod nounce {
    /// A BlockNonce is a 64-bit hash which proves (combined with the
    /// mix-hash) that a sufficient amount of computation has been carried
    /// out on a block.
    pub type BlockNounce = [u8; 8];

}


///
///
///
/*
struct BlockHeader {
    ParentHash: super::block::super::super::super::
    UncleHash   common.Hash    `json:"sha3Uncles"       gencodec:"required"`
    Coinbase    common.Address `json:"miner"            gencodec:"required"`
    Root        common.Hash    `json:"stateRoot"        gencodec:"required"`
    TxHash      common.Hash    `json:"transactionsRoot" gencodec:"required"`
    ReceiptHash common.Hash    `json:"receiptsRoot"     gencodec:"required"`
    Bloom       Bloom          `json:"logsBloom"        gencodec:"required"`
    Difficulty  *big.Int       `json:"difficulty"       gencodec:"required"`
    Number      *big.Int       `json:"number"           gencodec:"required"`
    GasLimit    uint64         `json:"gasLimit"         gencodec:"required"`
    GasUsed     uint64         `json:"gasUsed"          gencodec:"required"`
    Time        *big.Int       `json:"timestamp"        gencodec:"required"`
    Extra       []byte         `json:"extraData"        gencodec:"required"`
    MixDigest   common.Hash    `json:"mixHash"          gencodec:"required"`
    Nonce       BlockNonce     `json:"nonce"            gencodec:"required"`
}


/// Block with pre-commits.
# [derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockProof {
/// Block.
pub block: Block,
/// List of pre-commits for the block.
pub precommits: Vec <Precommit >,
}*/

# [cfg(test)]
mod tests {
# [test]
fn test_block() {}
}