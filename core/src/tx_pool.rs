use blockchain::block_chain;
use gen_pool::{Pool, Poolable, PoolError, ScoreRecord};
use transaction::*;

use std::sync::Mutex;

pub struct TXPoolConfig {
    pool_init_size: usize,
    name: String,
    chs: Vec<String>
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
        let config = TXPoolConfig::load();
        Mutex::new(Pool::new(config.name(), config.pool_size(), block_chain::cur_round() + 1))
    };
}