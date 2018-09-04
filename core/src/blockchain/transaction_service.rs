use block::Block;
use common::hash::Hash;
use db::gen_db::RocksDB;
use db::manager::DBManager;
use mpt::trie::*;
use transaction::Transaction;

pub struct TransactionService {
    db: RocksDB
}

impl TransactionService {
    pub fn new() -> Self {
        let mut db_manager = DBManager::default();
        TransactionService {
            db: db_manager.get_db("transaction")
        }
    }

    pub fn fetch_transaction_in_block(&self, block: &Block, hash: Hash) -> Option<Transaction> {
        let trie: Trie<Transaction> = Trie::load(block.txs_root(), &self.db);
        trie.get(&hash.to_vec())
    }

    pub fn fetch_all_transactions_in_block(&self, block: &Block) -> Vec<Transaction> {
        let trie: Trie<Transaction> = Trie::load(block.txs_root(), &self.db);
        trie.traversal()
    }
}