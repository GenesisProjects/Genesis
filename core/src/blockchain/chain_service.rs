//! Basic operation of committed block chain
//! Including reading/verification/synchronization
//!
//!

use account::Account;
use block::Block;
use transaction::Transaction;

use common::address::Address;
use db::manager::DBManager;
use mpt::node::TrieKey;
use mpt::trie::Trie;

pub enum ChainServiceError {
    MissingData
}

pub struct ChainService {

}

impl ChainService {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn get_last_block_header(&self) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    pub fn get_last_block_account(&self, addr: Address) -> Result<Account, ChainServiceError> {
        self.get_last_block_header().and_then(|block_header| {
            let mut shared_db_manager = DBManager::default();
            let account_db = shared_db_manager.get_db("state");
            let trie: Trie<Account> = Trie::load(block_header.account_root.clone(), &account_db);
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