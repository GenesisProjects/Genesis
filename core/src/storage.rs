use common::hash::*;
use common::address::Address;

use std::collections::HashMap;

pub type CHUNK = [u8; 32];

pub enum StorageError {

}

pub struct Storage {
    root: Hash,
    account_addr: Address,
    data: HashMap<Hash, CHUNK>
}

pub struct StorageCache {
    data: HashMap<Hash, CHUNK>,
    total_storage_alloc: usize,
    total_storage_free: usize,
}

impl StorageCache {
    pub fn new() -> Self {
        StorageCache {
            data: HashMap::new(),
            total_storage_alloc: 0,
            total_storage_free: 0,
        }
    }

    pub fn commit(self, to: &mut Storage) -> Result<Hash, StorageError> {
        unimplemented!()
    }

    pub fn add(&mut self, key: &Hash, value: CHUNK) -> Result<(), StorageError> {
        unimplemented!()
    }

    pub fn update(&mut self, key: &Hash, value: CHUNK) -> Result<(), StorageError> {
        unimplemented!()
    }

    pub fn read(&self, key: &Hash) -> Result<CHUNK, StorageError> {
        unimplemented!()
    }

    pub fn write(&mut self, key: &Hash, value: CHUNK) -> Result<(), StorageError> {
        unimplemented!()
    }

    pub fn delete(&mut self, key: &Hash) -> Result<(), StorageError> {
        unimplemented!()
    }
}