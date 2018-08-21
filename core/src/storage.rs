use super::mpt::trie::Trie;

use common::address::Address;
use common::hash::*;
use rlp::RLPSerialize;
use rlp::types::*;

use std::collections::HashMap;
use std::ops::Deref;

pub const CHUNK_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct CHUNK([u8; CHUNK_SIZE]);

impl Deref for CHUNK {
    type Target = [u8; CHUNK_SIZE];
    fn deref(&self) -> &[u8; CHUNK_SIZE] {
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

#[derive(Debug, Clone)]
pub struct Storage<'db> {
    trie: Trie<'db, CHUNK>,
    account_addr: Address,
}

impl<'db> Storage<'db> {
    pub fn get(&self, key: Hash) -> Option<CHUNK> {
        let vec = (&key[..]).to_vec();
        self.trie.get(&vec)
    }

    pub fn delete(&mut self, key: Hash) {
        let vec = (&key[..]).to_vec();
        self.trie.delete(&vec)
    }

    pub fn update(&mut self, key: Hash, chunk: CHUNK) {
        let vec = (&key[..]).to_vec();
        self.trie.update(&vec, &chunk)
    }
}

#[derive(Debug)]
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

    pub fn sync(self, to: &mut Storage) -> Result<Hash, StorageError> {
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