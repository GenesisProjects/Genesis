pub mod genesis;

pub mod block_chain {
    use chrono::{DateTime, Utc};

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
}