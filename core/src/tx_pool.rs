use gen_pool::{Pool, Poolable, PoolError, ScoreRecord};
use std::sync::Mutex;
use transaction::*;

impl Poolable for Transaction {
    fn score(&self) -> ScoreRecord {
        unimplemented!()
    }

    fn hash(&self) -> [u8; 32] {
        unimplemented!()
    }

    fn round(&self) -> usize {
        unimplemented!()
    }

    fn verify(&self) -> Result<(), PoolError> {
        unimplemented!()
    }
}

lazy_static! {
    pub static ref TX_POOL: Mutex<Pool<Transaction>> = {
        Mutex::new(Pool::new("tx_pool".into(), 1024 * 1024 * 16, 0))
    };
}