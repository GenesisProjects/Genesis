pub mod genesis;

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