use account::Account;
use block::Block;
use common::hash::Hash;
use db::gen_db::RocksDB;
use db::manager::DBManager;
use mpt::trie::*;

pub struct AccountService {
    db: RocksDB
}

impl AccountService {
    pub fn new() -> Self {
        let mut db_manager = DBManager::default();
        AccountService {
            db: db_manager.get_db("account")
        }
    }

    pub fn fetch_account_in_block(&self, block: &Block, root: Hash) -> Option<Account> {
        let trie: Trie<Account> = Trie::load(block.account_root(), &self.db);
        trie.get(&root.to_vec())
    }
}