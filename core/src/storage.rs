use super::mpt::trie::Trie;

use common::address::Address;
use common::hash::*;
use db::manager::*;
use rlp::RLPSerialize;
use rlp::types::*;

use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone)]
pub struct CHUNK([u8; 32]);

impl Deref for CHUNK {
    type Target = [u8; 32];
    fn deref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl RLPSerialize for CHUNK {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) ->  Result<Self, RLPError> {
        unimplemented!()
    }
}

pub enum StorageError {

}

pub struct Storage {
    trie: Trie<CHUNK>,
    account_addr: Address,
}

impl DBManagerOP for Storage {
    fn put<T: RLPSerialize>(&self, value: &T) -> Hash {
        unimplemented!()
    }

    fn delete(&self, key: &Vec<u8>) {
        unimplemented!()
    }

    fn get<T: RLPSerialize>(&self, key: &Vec<u8>) -> Option<T> {
        unimplemented!()
    }

    fn get_node<T: RLPSerialize>(&self, value: &T) -> Option<T> {
        unimplemented!()
    }
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

    pub fn add(&mut self, key: &Hash, value: CHUNK, storage: &Storage) -> Result<(), StorageError> {
        unimplemented!()
    }

    pub fn update(&mut self, key: &Hash, value: CHUNK, storage: &Storage) -> Result<(), StorageError> {
        unimplemented!()
    }

    pub fn read(&self, key: &Hash, storage: &Storage) -> Result<CHUNK, StorageError> {
        unimplemented!()
    }

    pub fn write(&mut self, key: &Hash, value: CHUNK, storage: &Storage) -> Result<(), StorageError> {
        unimplemented!()
    }

    pub fn delete(&mut self, key: &Hash, storage: &Storage) -> Result<(), StorageError> {
        unimplemented!()
    }
}