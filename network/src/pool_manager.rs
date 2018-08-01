use gen_core::transaction::Transaction;

use std::sync::Mutex;

use super::pool::*;

lazy_static! {
    pub static ref SHARED_POOL_MANAGER: Mutex<PoolManager> = {
        Mutex::new(PoolManager::new())
    };
}

impl super::pool::Poolable for Transaction {
    fn empty_obj() -> Self {
        unimplemented!()
    }

    fn unique_id(&self) -> &String {
        unimplemented!()
    }
}

pub struct PoolManager {
    transaction_pool: Pool<Transaction>,
}

impl PoolManager {
    pub fn new() -> Self {
        unimplemented!()
    }

    fn pooling(&mut self, transaction: Transaction) {
        self.transaction_pool.obtain().as_mut().unwrap().replace(transaction);
    }
}