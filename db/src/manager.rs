extern crate common;
extern crate rlp;

use self::common::hash::SerializableAndSHA256Hashable;
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

    fn put<T: RLPSerialize>(&self, key: &Vec<u8>, value: &T) -> Result<DBResult, DBError>;
    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Result<T, DBError>;

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

    fn put<T: RLPSerialize>(&self, key: &Vec<u8>, value: &T) -> Result<DBResult, DBError> {
        let rlp = match value.serialize() {
            Ok(r) => r,
            Err(_) => panic!("")
        };
        let encoded_rlp = SHARED_ENCODER.lock().unwrap().encode(&rlp);
        CAHCE.lock().unwrap().insert(key.clone(), encoded_rlp);
        Ok(DBResult::DBUpdateSuccess)
    }
    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Result<T, DBError> {
        let result = CAHCE.lock().unwrap().get(key).unwrap().clone();
        let rlp = Decoder::decode(&result).unwrap();
        match T::deserialize(&rlp) {
            Ok(r) => Ok(r),
            Err(_) => Err(DBError::DBFetchError { msg:"" })
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

    fn put<T: RLPSerialize>(&self, key: &Vec<u8>, value: &T) -> Result<DBResult, DBError> {
        Err(DBError::DBUpdateError { msg: "Unknown Err" })
    }
    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Result<T, DBError> {
        Err(DBError::DBFetchError { msg: "Unknown Err" })
    }

    fn show_status(&self) -> Result<DBStatus, DBError> {
        Err(DBError::DBStatusError { msg: "Unknown Err" })
    }
}