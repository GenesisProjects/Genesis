use common::hash::*;
use rlp::{RLPSerialize, decoder::Decoder};
use gen_rocksdb::*;
use ::rocksdb::{DB};
use config::*;
use std::path::Path;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;


pub struct DBManager {
    config: HashMap<String, Value>,
    dbs: HashMap<String, RocksDB>,
//    db: RocksDB
}

static DB_NAME: &'static str = "db/_trie_db";

impl DBManager {
    pub fn new() -> Self {
        let mut settings = Config::default();

        let path = Path::new("../config/application.json");
//
        settings.merge(File::from(path)).unwrap();

        let config = settings.get_table("db").unwrap();

        DBManager {
            config,
            dbs: HashMap::new()
        }
    }

    pub fn get_db(&mut self, key: &str) -> &RocksDB {
        let conf = self.config.get(key).unwrap().clone().into_table().unwrap();
        let path = conf.get("path").unwrap().clone().into_str().unwrap();
        let create_if_missing = conf.get("create_if_missing").unwrap().clone().into_bool().unwrap();
        let max_open_files = conf.get("max_open_files").unwrap().clone().into_int().unwrap() as i32;
        self.dbs.entry(key.to_string()).or_insert_with(|| {
            let db_config = DBConfig {
                create_if_missing,
                max_open_files
            };
            RocksDB::open(&db_config, &path[..])
        })
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
        unsafe {
            Mutex::new(DBManager::new())
        }
    };
}

///
///
///
//pub trait DBManagerOP {
//    fn put<T: RLPSerialize>(&self, value: &T) -> Hash;
//    fn delete(&self, key: &Vec<u8>);
//    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T>;
//}
//
//impl DBManagerOP for DBManager {
//    fn put<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, value: &T) -> Hash {
//        let db = &self.db.db;
//        let (key, encoded_rlp) = value.encrype_sha256().unwrap();
//        db.put(&key, encoded_rlp.as_slice()).expect("db put error");
//        key
//    }
//
//    fn delete(&self, key: &Vec<u8>) {
//        let db= &self.db.db;
//        db.delete(key);
//    }
//
//    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
//        match &self.db.db.get(key).unwrap() {
//            Some(t) => {
//                let result = t.to_vec();
//                let t= Decoder::decode(&result).unwrap();
//                Some(T::deserialize(&t).unwrap())
//            },
//            None => None
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test_db() {
        let mut m = &SHARED_MANAGER;
        let mut c = m.lock().unwrap();
        let r = c.get_db("test");
        println!("{:?}", r);
    }

//    #[test]
//    fn db_insert() {
//        let test_str: String = String::from("test");
//        let db = &SHARED_MANAGER;
//        let r = db.lock().unwrap().put(&test_str);
//        println!("{:?}", r);
//    }
}