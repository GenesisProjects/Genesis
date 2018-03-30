extern crate common;
extern crate rlp;

use self::common::hash::SerializableAndSHA256Hashable;
use self::rlp::RLPSerialize;

use std::sync::Mutex;

pub enum DBResult {
    DBConnectSuccess,
    DBDisConnectSuccess,
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

// TODO:
lazy_static! {
    pub static ref SHARED_MANAGER: Mutex<DBManager> = {
        static mut conf: DBConfig = DBConfig {};
        unsafe { Mutex::new(DBManager{ config: &mut conf }) }
    };
}

///
///
///
pub trait DBManagerOP {
    fn connect(&self,config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError>;
    fn disconnect(&self) -> Result<DBResult, DBError>;

    fn put<T: RLPSerialize>(&self, key: &String, value: &T) -> Result<DBResult, DBError>;
    fn get<T: RLPSerialize>(&self, key: &String) -> Result<T, DBError>;

    fn show_status(&self) -> Result<DBStatus, DBError>;
}

impl DBManagerOP for DBManager {
    fn connect(&self,config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError> {
        Err(DBError::DBConnectError { msg: "Unknown Err" })
    }

    fn disconnect(&self) -> Result<DBResult, DBError> {
        Err(DBError::DBDisconnectError { msg: "Unknown Err" })
    }

    fn put<T: RLPSerialize>(&self, key: &String, value: &T) -> Result<DBResult, DBError> {
        Err(DBError::DBUpdateError { msg: "Unknown Err" })
    }
    fn get<T: RLPSerialize>(&self, key: &String) -> Result<T, DBError> {
        Err(DBError::DBFetchError { msg: "Unknown Err" })
    }

    fn show_status(&self) -> Result<DBStatus, DBError> {
        Err(DBError::DBStatusError { msg: "Unknown Err" })
    }
}