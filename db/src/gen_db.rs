use byteorder::{BigEndian, ReadBytesExt};
use common::hash::*;
use rlp::{RLPSerialize, decoder::Decoder};
use ::rocksdb::{DB, DBIterator, DBRawIterator, IteratorMode, Options};
use std::{error::Error, fmt, iter::Peekable, mem, mem::transmute, path::Path, sync::Arc};

/// Database implementation on top of [`RocksDB`](https://rocksdb.org)
/// backend.
///
/// `RocksDB` is an embedded database for key-value data, which is optimized for fast storage.
/// This structure is required to potentially adapt the interface to
/// use different databases.

fn num_to_bytes(num: u64) -> [u8; 8] {
    let num_key_bytes: [u8; 8] = unsafe { transmute(num.to_be()) };
    num_key_bytes
}

fn bytes_to_num(bytes: Vec<u8>) -> Option<u64> {
    if bytes.len() != 8 {
        None
    } else {
        Some((&bytes[..]).read_u64::<BigEndian>().unwrap())
    }
}

#[derive(Clone)]
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

pub trait TrieNodeDBOP {
    fn put<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, value: &T) -> Hash;
    fn delete(&self, key: &Vec<u8>);
    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T>;
}

pub trait BlockDBOP {
    fn set_block_at_num<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, block: &T, num: u64) -> Hash;
    fn forward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator;
    fn backward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator;
    fn raw_iter<T: RLPSerialize>(&self, num: u64) -> DBRawIterator;
}

impl TrieNodeDBOP for RocksDB {
    fn put<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, value: &T) -> Hash {
        let db = &self.db;
        let (key, encoded_rlp) = value.encrype_sha256().unwrap();
        db.put(&key, encoded_rlp.as_slice()).expect("db put error");
        key
    }

    fn delete(&self, key: &Vec<u8>) {
        let db= &self.db;
        db.delete(key);
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
        match &self.db.get(key).unwrap() {
            Some(t) => {
                let result = t.to_vec();
                let t = Decoder::decode(&result).unwrap();
                Some(T::deserialize(&t).unwrap())
            },
            None => None
        }
    }
}

impl BlockDBOP for RocksDB {
    fn set_block_at_num<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, block: &T, num: u64) -> Hash {
        let num_key_bytes: [u8; 8] = unsafe { transmute(num.to_be()) };
        let db = &self.db;
        let (key, encoded_rlp) = block.encrype_sha256().unwrap();
        db.put(&num_key_bytes[..], encoded_rlp.as_slice()).expect("db put error");
        key
    }

    fn forward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator {
        self.db.iterator(IteratorMode::Start)
    }

    fn backward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator {
        self.db.iterator(IteratorMode::End)
    }

    fn raw_iter<T: RLPSerialize>(&self, num: u64) -> DBRawIterator {
        let mut iter = self.db.raw_iterator();
        let key = num_to_bytes(num);
        iter.seek(&key[..]);
        iter
    }
}

pub trait BlockDeRef {
    fn num(&self) -> Option<u64>;
    fn block<T: RLPSerialize>(&self) -> Option<T>;
}

impl BlockDeRef for DBRawIterator {
    fn num(&self) -> Option<u64> {
        if self.valid() {
            self.key().and_then(|bytes| {
                bytes_to_num(bytes)
            })
        } else {
            None
        }
    }

    fn block<T: RLPSerialize>(&self) -> Option<T> {
        if self.valid() {
            self.value().and_then(|v| {
                let t = Decoder::decode(&v).unwrap();
                Some(T::deserialize(&t).unwrap())
            })
        } else {
            None
        }
    }
}

impl fmt::Debug for RocksDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RocksDB(..)")
    }
}
