use common::address::Address;
use common::hash::Hash;
use rlp::RLPSerialize;
use rlp::types::*;
///
///
///
#[derive(Debug)]
pub struct Log {
    /// address of the contract that generated the event
    pub contract_address: Address,
    /// list of topics provided by the contract.
    pub topics: Vec<Hash>,
    /// supplied by the contract, usually ABI-encoded
    pub data: Vec<u8>,

    /// block in which the transaction was included
    pub block_number: u64,
    /// hash of the transaction
    pub tx_hash: Hash,
    /// index of the transaction in the block
    pub tx_index: u32,
    /// hash of the block in which the transaction was included
    pub block_hash: Hash,
    /// index of the log in the receipt
    pub index: u32
}

impl RLPSerialize for Log {
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
    fn test_log() {}
}