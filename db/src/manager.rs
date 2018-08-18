use common::hash::*;
use rlp::{RLPSerialize, decoder::Decoder};
use gen_rocksdb::*;
use ::rocksdb::{DB};

use std::sync::Mutex;
use std::collections::HashMap;


pub struct DBManager {
    config: &'static mut DBConfig,
    dbs: HashMap<String, RocksDB>
}

static DB_NAME: &'static str = "_trie_db";

impl DBManager {
    pub fn get_db(&self) -> RocksDB {
//        self.dbs.entry(key.to_string()).or_insert_with(|| {
//            RocksDB::open(self.config, DB_NAME)
//        })
        RocksDB::open(self.config, "_trie_db")
    }
    pub fn connect(&self, config: & DBConfig) -> Result<(RocksDB, DBResult), DBError> { unimplemented!() }

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
    fn put<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, value: &T) -> Hash {
        let db = &self.get_db().db;
        let (key, encoded_rlp) = value.encrype_sha256().unwrap();
        db.put(&key, encoded_rlp.as_slice()).expect("db put error");

        key
    }

    fn delete(&self, key: &Vec<u8>) {
        let db= &self.get_db().db;
        db.delete(key);
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
        let result = &self.get_db().db.get(key).unwrap().unwrap().to_vec();
        let t= Decoder::decode(&result).unwrap();
        Some(T::deserialize(&t).unwrap())
    }

    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_insert() {
        let test_str: String = String::from("test");
        let db = &SHARED_MANAGER;
        let r = db.lock().unwrap().put(&test_str);
        println!("{:?}", r);
    }
}