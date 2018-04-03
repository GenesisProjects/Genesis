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

#[cfg(not(mock))]
lazy_static! {
    static ref CAHCE: Mutex<HashMap<Vec<u8>, Vec<u8>>> = {
        Mutex::new(HashMap::new())
    };

    pub static ref SHARED_MANAGER: Mutex<DBManager> = {
        static mut conf: DBConfig = DBConfig {};
        unsafe { Mutex::new(DBManager{ config: &mut conf }) }
    };
}

#[cfg(mock)]
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
    fn connect(&self,config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError>;
    fn disconnect(&self) -> Result<DBResult, DBError>;

    fn put<T: RLPSerialize>(&self, value: &T) -> Hash;
    fn delete<T: RLPSerialize>(&self, value: &T) -> Hash;
    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T>;
    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T>;
    fn show_status(&self) -> Result<DBStatus, DBError>;
}

#[cfg(not(mock))]
impl DBManagerOP for DBManager {
    fn connect(&self,config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError> {
        Err(DBError::DBConnectError { msg: "Unknown Err" })
    }

    fn disconnect(&self) -> Result<DBResult, DBError> {
        Err(DBError::DBDisconnectError { msg: "Unknown Err" })
    }

    fn delete<T: RLPSerialize>(&self, value: &T) -> Hash {
        let (key, encoded_rlp) = value.encrype_sha256().unwrap();
        key
    }

    fn put<T: RLPSerialize>(&self, value: &T) -> Hash {
        let (key, encoded_rlp) = value.encrype_sha256().unwrap();
        CAHCE.lock().unwrap().insert(key.to_vec(), encoded_rlp);
        key
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
        match CAHCE.lock().unwrap().get(key) {
            Some(v) => {
                let rlp = Decoder::decode(v).unwrap();
                match T::deserialize(&rlp) {
                    Ok(r) => Some(r),
                    Err(_) => None
                }
            },
            None => None
        }
    }

    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T> {
        let (key, encoded_rlp) = value.encrype_sha256().unwrap();
        match CAHCE.lock().unwrap().get(&key.to_vec()) {
            Some(v) => {
                let rlp = Decoder::decode(v).unwrap();
                match T::deserialize(&rlp) {
                    Ok(r) => Some(r),
                    Err(_) => None
                }
            },
            None => None
        }
    }

    fn show_status(&self) -> Result<DBStatus, DBError> {
        Err(DBError::DBStatusError { msg: "Unknown Err" })
    }
}

#[cfg(mock)]
impl DBManagerOP for DBManager {
    fn connect(&self,config: & DBConfig) -> Result<(&'static DBContext, DBResult), DBError> {
        Err(DBError::DBConnectError { msg: "Unknown Err" })
    }

    fn disconnect(&self) -> Result<DBResult, DBError> {
        Err(DBError::DBDisconnectError { msg: "Unknown Err" })
    }

    fn delete<T: RLPSerialize>(&self, value: &T) -> Hash {
        Err(DBError::DBDisconnectError { msg: "Unknown Err" })
    }

    fn put<T: RLPSerialize>(&self, value: &T) -> Hash {
        [0u8; 32]
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
        None
    }

    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T> {
        None
    }

    fn show_status(&self) -> Result<DBStatus, DBError> {
        Err(DBError::DBStatusError { msg: "Unknown Err" })
    }
}