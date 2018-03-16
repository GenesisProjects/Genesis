use commin::hash;

///
///
///
struct Block {
    /// Information schema version.
    version: [u8, 4],
    /// Height of the current block
    height: i32,
    /// Number of transactions in block.
    tx_count: u32,
    /// Hash link to the previous block in blockchain.
    prev_hash: & Hash,
    /// Root hash of [merkle tree](struct.Schema.html#method.block_txs) of current block
    /// transactions.
    tx_hash: & Hash,
    /// Hash of the current `exonum` state after applying transactions in the block.
    state_hash: & Hash,
}


/// Block with pre-commits.
# [derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockProof {
/// Block.
pub block: Block,
/// List of pre-commits for the block.
pub precommits: Vec <Precommit >,
}

# [cfg(test)]
mod tests {


# [test]
fn test_block() {}
}