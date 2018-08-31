use block::Block;
use db::gen_db::RocksDB;

use super::defines::ChainServiceError;

pub mod BlockService {
    use super::*;

    fn seek(num: u64, db: &RocksDB) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    fn genesis_block(db: &RocksDB) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    fn last_block(db: &RocksDB) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    fn next_block(cur: &Block, db: &RocksDB) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    fn previous_block(cur: &Block, db: &RocksDB) -> Result<Block, ChainServiceError> {
        unimplemented!()
    }

    fn max_height() -> u64 {
        unimplemented!()
    }
}