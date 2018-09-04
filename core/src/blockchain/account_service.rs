use account::Account;
use block::Block;
use common::address::Address;
use common::hash::Hash;
use db::gen_db::RocksDB;
use db::manager::DBManager;
use mpt::trie::*;

pub type ContractCodeStream = Vec<u8>;

pub struct AccountService {
    db: RocksDB,
    code_db: RocksDB
}

impl AccountService {
    pub fn new() -> Self {
        let mut db_manager = DBManager::default();
        AccountService {
            db: db_manager.get_db("account"),
            code_db: db_manager.get_db("code")
        }
    }

    pub fn fetch_account_in_block(&self, block: &Block, addr: Address) -> Option<Account> {
        let trie: Trie<Account> = Trie::load(block.account_root(), &self.db);
        trie.get(&addr.to_key().unwrap().to_vec())
    }

    pub fn fetch_code_in_block(&self, block: &Block, addr: Address) -> Option<ContractCodeStream> {
       unimplemented!()
    }
}