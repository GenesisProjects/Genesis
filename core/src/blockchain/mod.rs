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

/// Return current round base on timestamp and genesis timestamp
pub fn round(time: DateTime<Utc>) -> usize {
    unimplemented!()
}

/// Return self block chain length.
pub fn self_block_chain_len() -> usize {
    unimplemented!()
}

/// Return current estimated block chain length.
pub fn cur_block_chain_len() -> usize {
    unimplemented!()
}

/// Return last block transaction hash root, if the block chain has not been sync, return None
pub fn last_block_tx_root() -> Option<Hash> {
    unimplemented!()
}

/// Return top N validator from last block
pub fn validator_from_last_block(n: usize) -> Vec<Validator> {
    unimplemented!()
}