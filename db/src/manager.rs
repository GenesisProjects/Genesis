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
pub trait DBManager<'a> {
    fn connect(config: &'a DBConfig) -> Result<(DBContext, DBResult), DBError>;
    fn disconnect() -> Result<DBResult, DBError>;

    fn put<T: SHA256Hashable>(key: &'a String, value: &'a T) -> Result<DBResult, DBError>;
    fn get<T: SHA256Hashable>(key: &'a String) -> Result<T, DBError>;

    fn show_status() -> Result<DBStatus, DBError>;
}