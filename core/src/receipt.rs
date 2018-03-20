extern crate common;
extern crate num;

use self::common::hash::*;
use self::common::address::*;
use self::common::bloom::*;
use self::num::bigint::BigInt;
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
struct Receipt<'a> {
    /// Consensus fields
    pub post_state: Vec<u8>,
    pub status: ReceiptStatus,
    pub cumulative_gas_used: u64,
    pub logs_bloom: Option<Bloom<Log>>,
    pub logs: Option<Vec<&'a Log>>,

    // Implementation fields (don't reorder!)
    txhash: Option<Hash>,
    contract_address: Option<Address>,
    gas_used: u64
}

impl<'a> Receipt<'a> {
    /// NewReceipt creates a barebone transaction receipt, copying the init fields.
    pub fn new(root: &'a Vec<u8>, failed: bool, cumulative_gas_used: u64) -> Self {
        let r = Receipt {
            post_state: root.to_vec(),
            status: ReceiptStatus::ReceiptStatusUnknown,
            cumulative_gas_used: cumulative_gas_used,
            logs_bloom: None,
            logs: None,

            txhash: None,
            contract_address: None,
            gas_used: 0
        };
        r
    }
}
