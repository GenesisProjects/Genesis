pub mod account_service;
pub mod block_service;
pub mod chain_cache;
pub mod chain_service;
pub mod genesis;
pub mod transaction_service;

use validator::Validator;

use chrono::{DateTime, Utc};
use common::hash::Hash;

/// Return current round base on current time and genesis timestamp
pub fn cur_round() -> usize {
    unimplemented!()
}

/// Return current estimated round base on timestamp and genesis timestamp
pub fn estimated_round(time: DateTime<Utc>) -> usize {
    unimplemented!()
}

/// Return current round base on db record
pub fn round(time: DateTime<Utc>) -> usize {
    unimplemented!()
}

/// Return self block chain length.
pub fn block_chain_len() -> usize {
    unimplemented!()
}

/// Return estimated block chain length according to DB record.
pub fn estimated_block_chain_len() -> usize {
    unimplemented!()
}

/// Return last block transaction hash root
pub fn last_block_tx_root() -> Hash {
    unimplemented!()
}

/// Return last block header hash
pub fn last_block_hash() -> Hash {
    unimplemented!()
}

/// Return top N validator from last block
pub fn validator_from_last_block(n: usize) -> Vec<Validator> {
    unimplemented!()
}