use blockchain;
use common::hash::Hash;
use gen_pool::{Pool, Poolable, PoolError, ScoreRecord};
use std::sync::Mutex;
use transaction::*;

pub struct TXPoolConfig {
    pool_init_size: usize,
    name: String,
}

impl TXPoolConfig {
    pub fn load() -> Self {
        unimplemented!()
    }

    pub fn pool_size(&self) -> usize {
        self.pool_init_size
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl Poolable for Transaction {
    fn score(&self) -> ScoreRecord {
        unimplemented!()
    }

    fn hash(&self) -> Hash {
        unimplemented!()
    }

    fn round(&self) -> usize {
        blockchain::round(self.timestamp())
    }

    fn verify(&self) -> Result<(), PoolError> {
        unimplemented!()
    }
}

lazy_static! {
    pub static ref TX_POOL: Mutex<Pool<Transaction>> = {
        let config = TXPoolConfig::load();
        let pool = Pool::new(config.name(), config.pool_size(), blockchain::cur_round() + 1);
        Mutex::new(pool)
    };
}