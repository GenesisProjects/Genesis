//! Basic operation of committed block chain
//! Including reading/verification/synchronization
//!
//!

use account::Account;
use block::Block;
use transaction::Transaction;

use blockchain::account_service::AccountService;
use blockchain::block_service::BlockService;
use blockchain::transaction_service::TransactionService;

use common::address::Address;
use db::manager::DBManager;
pub use db::gen_db::{BlockDeRef, ChainDBOP, DBError, DBRawIterator, RocksDB};
use mpt::node::TrieKey;
use mpt::trie::Trie;

use super::defines::ChainServiceError;

pub struct ChainService {
    account_service: AccountService,
    block_service: BlockService,
    transaction_service: TransactionService
}


impl ChainService {
    pub fn new() -> Self {
        let mut db_manager = DBManager::default();
        ChainService {
            account_service: AccountService::new(),
            block_service: BlockService::new(),
            transaction_service: TransactionService::new()
        }
    }

    pub fn get_block_height(&self) -> u64 {
        let block: Block = self.block_service.last().block().unwrap();
        block.num()
    }

    pub fn get_last_block_header(&self) -> Result<Block, DBError> {
        self.block_service.last().block().ok_or_else(|| {
            DBError::new("Can not find the last block".into())
        })
    }

    pub fn get_last_block_account(&self, addr: Address) -> Result<Account, DBError> {
       unimplemented!()
    }

    pub fn get_last_block_account_nonce(&self, addr: Address) -> Result<u64, DBError> {
        unimplemented!()
    }

    pub fn get_transactions(&self, trie: Trie<Transaction>)
        -> Result<Vec<Transaction>, ChainServiceError> {
        unimplemented!()
    }

    pub fn replay_txs(&self, new_txs: Vec<Transaction>, old_account_root: Trie<Account>)
        -> Result<Trie<Account>, ChainServiceError> {
        unimplemented!()
    }
}

pub enum Validation {

}

pub struct ValidationBuilder {

}