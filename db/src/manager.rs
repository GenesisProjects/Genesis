use common::hash::*;
use rlp::{RLPSerialize, decoder::Decoder};
use gen_db::*;
use ::rocksdb::{DB};
use config::*;
use std::path::Path;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;


pub struct DBManager {
    config: HashMap<String, Value>,
}

static DB_NAME: &'static str = "db/_trie_db";

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
        let db_config = DBConfig {
            create_if_missing,
            max_open_files
        };
        RocksDB::open(&db_config, &path[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test_db() {
        let mut c = DBManager::default();
        let d = c.get_db("test");
    }

    #[test]
    fn test_db_insert() {
        let mut c = DBManager::default();
        let db = c.get_db("test");

        let test_str = String::from("test");
        let r = db.put(&test_str);
    }
}