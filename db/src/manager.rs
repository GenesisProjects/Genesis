extern crate common;
use self::common::hash::SHA256Hashable;

use types::{ DBContext, DBStatus, DBConfig };

pub enum DBResult {
    DBConnectSuccess,
    DBDisConnectSuccess,
}

pub enum DBError {
    DBConnectError{ msg: String },
    DBDisConnectSuccess { msg: String },
}

///
///
///
pub trait DBManager {
    fn connect(config: & DBConfig) -> Result<(DBContext, DBResult), DBError>;
    fn disconnect() -> Result<DBResult, DBError>;

    fn put<'a, T: SHA256Hashable<'a>>(key: &'a String, value: &'a T) -> Result<DBResult, DBError>;
    fn get<'a, T: SHA256Hashable<'a>>(key: &'a String) -> Result<T, DBError>;

    fn show_status() -> Result<DBStatus, DBError>;
}