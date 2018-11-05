use byteorder::{BigEndian, ReadBytesExt};
use common::hash::*;
use rlp::{RLPSerialize, decoder::Decoder};
pub use ::rocksdb::{DB, DBIterator, DBRawIterator, IteratorMode, Options};
use std::{error::Error, fmt, iter::Peekable, mem, mem::transmute, path::Path, sync::Arc};

/// Database implementation on top of [`RocksDB`](https://rocksdb.org)
/// backend.
///
/// `RocksDB` is an embedded database for key-value data, which is optimized for fast storage.
/// This structure is required to potentially adapt the interface to
/// use different databases.

#[derive(Debug, Clone)]
pub struct DBError {
    msg: String
}

impl DBError {
    pub fn new(msg: String) -> Self {
        DBError { msg: msg }
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

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
    fn put<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, value: &T) -> Result<Hash, DBError>;
    fn delete(&self, key: &Vec<u8>) -> Result<(), DBError>;
    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Result<Option<T>, DBError>;
}

pub trait ChainDBOP {
    fn set_block_at_num<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, block: &T, num: u64) -> Result<Hash, DBError>;
    fn delete_block_at_num<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, num: u64) -> Result<(), DBError>;
    fn forward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator;
    fn backward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator;
    fn raw_iter<T: RLPSerialize>(&self, num: u64) -> DBRawIterator;
}

impl TrieNodeDBOP for RocksDB {
    fn put<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, value: &T) -> Result<Hash, DBError> {
        let db = &self.db;
        let (key, encoded_rlp) = value.encrype_sha256().unwrap();
        db.put(&key, encoded_rlp.as_slice()).and_then(|_| {
            Ok(key)
        }).map_err(|e| {
            DBError::new(e.to_string())
        })
    }

    fn delete(&self, key: &Vec<u8>) -> Result<(), DBError> {
        let db= &self.db;
        db.delete(key).map_err(|e| {
            DBError::new(e.to_string())
        })
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Result<Option<T>, DBError> {
        self.db.get(key).map_err(|e| {
            DBError::new(e.to_string())
        }).and_then(|v| {
            match v {
                Some(t) => {
                    let result = t.to_vec();
                    Decoder::decode(&result).ok_or_else(|| {
                        DBError::new("The node is malformed".into())
                    }).and_then(|rlp| {
                        T::deserialize(&rlp).map_err(|e| {
                            DBError::new("RLP deserialize error".into())
                        })
                    }).and_then(|r| {
                        Ok(Some(r))
                    })
                },
                None => Ok(None)
            }
        })
    }
}

impl ChainDBOP for RocksDB {
    fn set_block_at_num<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, block: &T, num: u64) -> Result<Hash, DBError> {
        let num_key_bytes: [u8; 8] = num_to_bytes(num);
        let db = &self.db;
        let (key, encoded_rlp) = block.encrype_sha256().unwrap();
        db.put(&num_key_bytes[..], encoded_rlp.as_slice()).and_then(|_| {
            Ok(key)
        }).map_err(|e| {
            DBError::new(e.to_string())
        })
    }

    fn delete_block_at_num<T: RLPSerialize + SerializableAndSHA256Hashable>(&self, num: u64) -> Result<(), DBError> {
        let num_key_bytes: [u8; 8] = num_to_bytes(num);
        let db = &self.db;
        db.delete(&num_key_bytes[..]).map_err(|e| {
            DBError::new(e.to_string())
        })
    }

    fn forward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator {
        self.db.iterator(IteratorMode::Start)
    }

    fn backward_iter<T: RLPSerialize>(&self, num: u64) -> DBIterator {
        self.db.iterator(IteratorMode::End)
    }

    fn raw_iter<T: RLPSerialize>(&self, num: u64) -> DBRawIterator {
        if num == 0 {
            self.db.raw_iterator()
        } else {
            let mut iter = self.db.raw_iterator();
            let key = num_to_bytes(num);
            iter.seek(&key[..]);
            iter
        }
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
                Decoder::decode(&v).and_then(|rlp| {
                    T::deserialize(&rlp).ok()
                })
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
