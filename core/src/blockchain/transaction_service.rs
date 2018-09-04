use transaction::Transaction;
use db::gen_db::RocksDB;
use db::manager::DBManager;

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
}