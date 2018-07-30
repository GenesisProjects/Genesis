use common::hash::*;
use common::address::*;
use rlp::RLPSerialize;
use rlp::types::*;
use log::Log;

#[derive(Debug)]
enum ReceiptStatus {
    ReceiptStatusUnknown,
    ReceiptStatusFailedRLP,
    ReceiptStatusSuccessful,
    ReceiptStatusSuccessfulRLP(Vec<u8>)
}

/// Receipt represents the results of a transaction.
#[derive(Debug)]
struct Receipt {
    /// Consensus fields
    pub post_state: Vec<u8>,
    pub status: ReceiptStatus,
    pub cumulative_gas_used: u64,
    //pub logs_bloom: Option<Bloom<Log>>,
    pub logs: Option<Vec<Log>>,

    // Implementation fields (don't reorder!)
    txhash: Option<Hash>,
    contract_address: Option<Address>,
    gas_used: u64
}

impl Receipt {
    /// NewReceipt creates a barebone transaction receipt, copying the init fields.
    pub fn new(root: &Vec<u8>, failed: bool, cumulative_gas_used: u64) -> Self {
        let r = Receipt {
            post_state: root.to_vec(),
            status: ReceiptStatus::ReceiptStatusUnknown,
            cumulative_gas_used: cumulative_gas_used,
            //logs_bloom: None,
            logs: None,

            txhash: None,
            contract_address: None,
            gas_used: 0
        };
        r
    }
}

impl RLPSerialize for Receipt {
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
    fn test_receipt() {}
}
