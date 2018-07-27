extern crate common;
extern crate rlp;

use self::common::hash::{ SerializableAndSHA256Hashable, Hash };

use self::rlp::RLPSerialize;
use self::rlp::encoder::SHARED_ENCODER;
use self::rlp::decoder::Decoder;

use std::sync::Mutex;
use std::collections::HashMap;

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

}

pub struct DBManager {
    config: &'static mut DBConfig
}

impl DBManager {
    pub fn connect(&self,config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError> {
        unimplemented!()
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