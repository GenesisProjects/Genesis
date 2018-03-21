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
pub trait DBManager<'a, T: std::string::ToString> {
    fn connect(config: &'a DBConfig) -> Result<(DBContext, DBResult), DBError>;
    fn disconnect() -> Result<DBResult, DBError>;

    fn put<T>(key: &'a String, value: &'a T) -> Result<DBResult, DBError>;
    fn get<T>(key: &'a String) -> Result<T, DBError>;

    fn show_status() -> Result<DBStatus, DBError>;
}