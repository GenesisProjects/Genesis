use ::rocksdb::DB;
use common::hash::*;
use config::*;
use gen_db::*;
use rlp::{decoder::Decoder, RLPSerialize};

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref DB_MAP: Mutex<HashMap<String, RocksDB>> = {
        Mutex::new(HashMap::new())
    };
}

pub struct DBManager {
    config: HashMap<String, Value>
}

static mut DB_NAME: &'static str = "db/_trie_db";

impl DBManager {
    pub fn default() -> Self {
        let mut settings = Config::default();
        let path = Path::new("../config/application.json");
        settings.merge(File::from(path)).unwrap();
        let config = settings.get_table("db").unwrap();
        DBManager {
            config
        }
    }

    pub fn get_db(&mut self, key: &str) -> RocksDB {
        let conf = self.config.get(key).unwrap().clone().into_table().unwrap();
        let path = conf.get("path").unwrap().clone().into_str().unwrap();
        let create_if_missing = conf.get("create_if_missing").unwrap().clone().into_bool().unwrap();
        let max_open_files = conf.get("max_open_files").unwrap().clone().into_int().unwrap() as i32;
        let mut map = DB_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| {
            let db_config = DBConfig {
                create_if_missing,
                max_open_files,
            };
            RocksDB::open(&db_config, &path[..])
        }).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test_db() {
        let mut c = DBManager::default();
        let d = c.get_db("test");
        println!("{:?}", d);
    }

    #[test]
    fn test_db_insert() {
        let mut c = DBManager::default();
        let db = c.get_db("test");

        let test_str = String::from("test");
        let r = db.put(&test_str);
        println!("{:?}", r);
    }
}