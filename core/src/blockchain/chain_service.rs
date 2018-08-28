//! Basic operation of committed block chain
//! Including reading/verification/synchronization
//!
//!

use account::Account;
use block::Block;
use transaction::Transaction;

use common::address::Address;
use db::manager::SHARED_MANAGER;
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

    pub fn verify_block_header(&self) -> bool {
        self.verify_signature()
    }

    pub fn verify_signature(&self) -> bool {
        unimplemented!()
    }

    pub fn get_last_block_header(&self) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    pub fn get_last_block_account(&self, addr: Address) -> Result<Account, ChainServiceError> {
        self.get_last_block_header().and_then(|block_header| {
            let mut shared_db_manager = SHARED_MANAGER.lock().unwrap();
            let account_db = shared_db_manager.get_db("account");
            let trie: Trie<Account> = Trie::load(block_header.account_root.clone(), &account_db);
            match trie.get(&addr.text.into_bytes()) {
                Some(account) => Ok(account),
                None => Err(ChainServiceError::MissingData)
            }
        })
    }

    pub fn replay_txs(&self) {
        unimplemented!()
    }
}