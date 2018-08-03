extern crate common;
extern crate rlp;

use self::common::hash::Hash;
use self::rlp::RLPSerialize;
use gen_rocksdb::*;

use std::sync::Mutex;

pub enum DBResult {
    DBConnectSuccess,
    DBDisconnectSuccess,
    DBUpdateSuccess,
    DBFetchSuccess,
    DBStatusSuccess,
}

pub enum DBError {
    DBConnectError{ msg: &'static str },
    DBDisconnectError { msg: &'static str },
    DBUpdateError { msg: &'static str },
    DBFetchError { msg: &'static str },
    DBStatusError { msg: &'static str },
}

pub struct DBContext {

}

pub struct DBStatus {

}

pub struct DBConfig {
    pub create_if_missing: bool,
    pub max_open_files: i32,
    pub path: String
}

pub struct DBManager {
    config: &'static mut DBConfig
}

impl DBManager {
    pub fn connect(&self, config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError> {
        RocksDB::open(config)
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
        static mut conf: DBConfig = DBConfig {};
        unsafe { Mutex::new(DBManager{ config: &mut conf }) }
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
    fn delete(&self, key: &Vec<u8>) {
        unimplemented!()
    }

    fn put<T: RLPSerialize>(&self, value: &T) -> Hash {
        unimplemented!()
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
        unimplemented!()
    }

    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T> {
        unimplemented!()
    }
}