use block::Block;
use db::gen_db::{BlockDeRef, ChainDBOP, DBError, DBRawIterator, RocksDB};
use db::manager::DBManager;

pub struct BlockService {
    db: RocksDB
}

impl BlockService {
    pub fn new() -> Self {
        let mut db_manager = DBManager::default();
        BlockService {
            db: db_manager.get_db("block")
        }
    }

    pub fn seek(&self, num: u64) -> DBRawIterator {
        self.db.raw_iter::<Block>(num)
    }

    pub fn last(&self) -> DBRawIterator {
        let mut iter = self.db.raw_iter::<Block>(0);
        iter.seek_to_last();
        iter
    }

    pub fn genesis(&self) -> DBRawIterator {
        let mut iter = self.db.raw_iter::<Block>(0);
        iter.seek_to_first();
        iter
    }
}