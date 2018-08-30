//! Basic operation of committed block chain
//! Including reading/verification/synchronization
//!
//!

use account::Account;
use block::Block;
use transaction::Transaction;

use common::address::Address;
use db::manager::DBManager;
use db::gen_db::RocksDB;
use mpt::node::TrieKey;
use mpt::trie::Trie;

pub enum ChainServiceError {
    MissingData
}

pub struct ChainService {
    accounts_db: RocksDB,
    blocks_db: RocksDB,
    txs_db: RocksDB
}

impl ChainService {
    pub fn new() -> Self {
        let mut db_manager = DBManager::default();
        ChainService {
            accounts_db: db_manager.get_db("state"),
            blocks_db: db_manager.get_db("block"),
            txs_db: db_manager.get_db("transaction")
        }
    }

    fn accounts_db(&self) -> RocksDB {
        self.accounts_db.clone()
    }

    fn blocks_db(&self) -> RocksDB {
        self.blocks_db.clone()
    }

    fn txs_db(&self) -> RocksDB {
        self.txs_db.clone()
    }

    pub fn get_block_height(&self) -> u64 {
        unimplemented!()
    }

    pub fn get_last_block_header(&self) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    pub fn get_last_block_account(&self, addr: Address) -> Result<Account, ChainServiceError> {
        self.get_last_block_header().and_then(|block_header| {
            let db = self.accounts_db();
            let trie: Trie<Account> = Trie::load(block_header.account_root.clone(), &db);
            match trie.get(&addr.text.into_bytes()) {
                Some(account) => Ok(account),
                None => Err(ChainServiceError::MissingData)
            }
        })
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