use ::rocksdb::{DB, Options};
use std::{error::Error, fmt, iter::Peekable, mem, path::Path, sync::Arc};
use rlp::{RLPSerialize, decoder::Decoder};
use common::hash::*;

/// Database implementation on top of [`RocksDB`](https://rocksdb.org)
/// backend.
///
/// `RocksDB` is an embedded database for key-value data, which is optimized for fast storage.
/// This structure is required to potentially adapt the interface to
/// use different databases.

pub struct RocksDB {
    pub db: Arc<::rocksdb::DB>,
}

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
}

impl DBConfig {
    fn to_rocksdb(&self) -> Options {
        let mut defaults = Options::default();
        defaults.create_if_missing(self.create_if_missing);
        defaults.set_max_open_files(self.max_open_files);
        defaults
    }
}


impl RocksDB {
    pub fn open(options: &DBConfig, path: &str) -> Self {
        let db = DB::open(&options.to_rocksdb(), path).unwrap();
        Self { db: Arc::new(db) }
    }
}

impl fmt::Debug for RocksDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RocksDB(..)")
    }
}
