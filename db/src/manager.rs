extern crate common;
extern crate rlp;

use self::common::hash::SerializableAndSHA256Hashable;
use self::rlp::RLPSerialize;

use std::sync::Mutex;
use types::{ DBContext, DBStatus, DBConfig };

pub enum DBResult {
    DBConnectSuccess,
    DBDisConnectSuccess,
}

pub enum DBError {
    DBConnectError{ msg: String },
    DBDisConnectSuccess { msg: String },
}

pub struct DBManager {

}

// TODO:
lazy_static! {
    pub static ref SHARED_MANAGER: Mutex<DBManager> = {
        Mutex::new(DBManager{})
    };
}

///
///
///
pub trait DBManagerOP {
    fn connect(config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError>;
    fn disconnect() -> Result<DBResult, DBError>;

    fn put<T: RLPSerialize>(key: &String, value: &T) -> Result<DBResult, DBError>;
    fn get<T: RLPSerialize>(key: &String) -> Result<T, DBError>;

    fn show_status() -> Result<DBStatus, DBError>;
}