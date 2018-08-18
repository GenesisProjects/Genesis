use common::hash::*;
use rlp::RLPSerialize;
use gen_rocksdb::*;
use ::rocksdb::{DB};

use std::sync::Mutex;
use std::collections::HashMap;


pub struct DBManager {
    config: &'static mut DBConfig,
    dbs: HashMap<String, RocksDB>
}

impl DBManager {
    pub fn getDb(&self, key: &str) -> &RocksDB {
        
    }
    pub fn connect(&self, config: & DBConfig) -> Result<(RocksDB, DBResult), DBError> {
        let db = RocksDB::open(config);
        Ok(( db, DBResult::DBConnectSuccess ))
    }

    pub fn disconnect(&self) -> Result<DBResult, DBError> {
        unimplemented!()
    }

    pub fn show_status(&self) -> Result<DBStatus, DBError> {
        unimplemented!()
    }
}

lazy_static! {
    //TODO:
    pub static ref SHARED_MANAGER: Mutex<DBManager> = {
        static mut conf: DBConfig = DBConfig {
            create_if_missing: false,
            max_open_files: 32
        };
        unsafe {
            Mutex::new(DBManager{
                config: &mut conf,
                dbs: HashMap::new()
            })
        }
    };
}

///
///
///
pub trait DBManagerOP {
    fn put<T: RLPSerialize>(&self, value: &T) -> Hash;
    fn delete(&self, key: &Vec<u8>);
    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T>;
    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T>;
}

impl DBManagerOP for DBManager {
    fn put<T: RLPSerialize>(&mut self, value: &T) -> Hash {
        let (mut db, _) = self.connect(self.config).unwrap();
    }

    fn delete(&mut self, key: &Vec<u8>) {
        unimplemented!()
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
        let (db, _) = self.connect(self.config).unwrap();
        db.get(key.as_slice())
    }

    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T> {
        unimplemented!()
    }
}

