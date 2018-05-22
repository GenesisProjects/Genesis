use gen_core::block::Block;
use gen_core::transaction::Transaction;

use rlp::decoder::Decoder;
use rlp::types::RLP;
use rlp::RLPSerialize;

use std::sync::Mutex;

use super::pool::*;

lazy_static! {
    pub static ref SHARED_POOL_MANAGER: Mutex<PoolManager> = {
        Mutex::new(PoolManager::new())
    };
}

impl super::pool::Poolable for Block {
    fn empty_obj() -> Self {
        unimplemented!()
    }

    fn unique_id(&self) -> &String {
        unimplemented!()
    }
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
    block_pool: Pool<Block>,
    transaction_pool: Pool<Transaction>,
}

impl PoolManager {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn accept(&mut self, data: &Vec<u8>) {
        let rlp = Decoder::decode(data);
        rlp.and_then(|rlp| {
            self.pooling(&rlp);
            Some(rlp)
        });
    }

    fn pooling(&mut self, rlp: &RLP) {
        let tag = PoolManager::get_rlp_tag(rlp);
        match tag.as_ref() {
            "block" => {
                Block::deserialize(rlp).and_then(|block| {
                    self.block_pool.obtain().as_mut().unwrap().replace(block);
                    Ok("")
                });
            },
            "transaction" => {
                Transaction::deserialize(rlp).and_then(|tx| {
                    self.transaction_pool.obtain().as_mut().unwrap().replace(tx);
                    Ok("")
                });
            },
            _ => {}
        };
    }

    fn get_rlp_tag(rlp: &RLP) -> String {
        match *rlp {
            RLP::RLPList { ref list } => {
                let tag = list.first().and_then(|result| {
                     if let RLP::RLPItem { ref value } = result.clone() {
                         Some(String::from_utf8_lossy(value).to_string())
                     } else { None }
                });
                tag.unwrap_or("Error".to_string())
            },
            RLP::RLPItem { ref value } => { "Error".to_string() }
        }
    }
}