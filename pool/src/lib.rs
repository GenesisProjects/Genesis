extern crate common;
extern crate gen_core;
extern crate gen_message;
extern crate slab;

use common::address::*;
use common::hash::*;
use common::observe::*;
use slab::Slab;
use gen_core::blockchain::chain_service;
use gen_core::transaction::Transaction;
use gen_message::*;

use std::collections::HashMap;
use std::cmp::Ordering;
use std::mem;

pub enum PoolError {
    DBError(String),
    Duplicate(Hash),
    NonceError,
    Locked,
}

pub trait Poolable {
    /// Unique hash id
    fn hash(&self) -> Hash;
    /// Account address with this transaction
    fn account(&self) -> Address;
    /// Nonce for this transaction
    fn nonce(&self) -> u64;
    /// Verify if the object is valid
    fn verify(&self) -> Result<(), PoolError>;
}

impl Poolable for Transaction {
    fn hash(&self) -> Hash {
        self.hash()
    }

    fn nonce(&self) -> u64 {
        self.nonce()
    }

    fn verify(&self) -> Result<(), PoolError> {
        self.check()
    }

    fn account(&self) -> Address {
        self.sender()
    }
}


/// Object pool, which uses slab allocator to manage memory
/// Should notify all channels after added a new object
pub struct Pool<T> where T: Poolable {
    name: String,
    channels: Vec<String>,
    slab: Slab<T>,
    slab_key_map: HashMap<Hash, usize>,
    nonce_map: HashMap<Hash, u64>,
    chain_service: chain_service::ChainService,
    locked: bool
}

impl<T> Pool<T> where T: Poolable {
    /// Init pool with capacity
    pub fn new(name: String, size: usize, next_round: usize) -> Self {
        Pool {
            name: name,
            channels: vec![],
            slab: Slab::with_capacity(size),
            slab_key_map: HashMap::new(),
            nonce_map: HashMap::new(),
            chain_service: chain_service::ChainService::new(),
            locked: false
        }
    }

    /// Add controller by channel name to notify
    #[inline]
    pub fn add_channel(&mut self, name: String) {
        self.channels.push(name);
    }

    /// Remove controller by channel name
    #[inline]
    pub fn remove_channel(&mut self, index: usize) {
        self.channels.remove(index);
    }

    /// Find channel index by name
    #[inline]
    pub fn channel_index(&mut self, name: String) -> usize {
        self.channels
            .iter()
            .enumerate()
            .find(|r| r.1.to_owned() == name.to_owned())
            .unwrap()
            .0
    }

    /// Notify all channels if recieve a new transaction with message: "new_tx"
    #[inline]
    fn notify_new_tx_recieved(&self) {
        self.channels
            .iter()
            .map(|ch| {
                MESSAGE_CENTER.lock()
                    .unwrap()
                    .send(
                        ch.to_string(),
                        Message::new(0, "new_tx".to_string()),
                    );
            });
    }

    /// If the object exist in pending pool
    #[inline]
    pub fn exist(&self, obj: &T) -> bool {
        let hash = obj.hash();
        self.slab_key_map.get(&hash).is_some()
    }

    /// Check if the nonce is correct
    fn verify_nonce(&self, obj: &T) -> Result<bool, chain_service::DBError> {
        self.get_account_nonce(obj.account()).and_then(|nonce| {
            Ok(nonce == obj.nonce())
        })
    }

    /// Get the account nonce from the last block in block chain
    fn get_account_nonce(&self, account_addr: Address) -> Result<u64, chain_service::DBError> {
        self.chain_service.get_last_block_account_nonce(account_addr)
    }

    /// Check if the transaction is valid
    fn check(&self, obj: &T) -> Result<(), PoolError> {
        match self.verify_nonce(&obj) {
            Ok(r) => {
                if r {
                    obj.verify()
                } else {
                    Err(PoolError::NonceError)
                }
            }
            Err(e) => Err(PoolError::DBError(e.msg()))
        }
    }

    /// Insert an object into the pending pool
    /// Key must be type of [Hash]
    /// If the object has already been stored, do nothing
    #[inline]
    pub fn push(&mut self, obj: T) -> Result<(), PoolError> {
        if self.locked {
            Err(PoolError::Locked)
        } else if self.exist(&obj) {
            Err(PoolError::Duplicate(obj.hash()))
        } else if let Err(e) = self.check(&obj) {
            Err(e)
        } else {
            let hash = obj.hash();
            let slab_key = self.slab.insert(obj);
            self.slab_key_map.insert(hash, slab_key);
            self.notify_new_tx_recieved();
            Ok(())
        }
    }

    /// Fetch a enqueued object.
    /// The object will be consumed
    #[inline]
    pub fn fetch(&mut self, hash: Hash) -> Option<T> {
       match self.slab_key_map.remove(&hash) {
           Some(r) => {
               Some(self.slab.remove(r))
           },
           None => None
       }
    }

    /// Clear the pending object pool
    #[inline]
    fn clear(&mut self) -> Result<(), PoolError> {
        self.slab.clear();
        self.slab_key_map.clear();
        Ok(())
    }

    /// Lock the pool
    #[inline]
    pub fn lock(&mut self) {
        self.locked = true
    }

    /// Unlock the pool
    #[inline]
    pub fn unlock(&mut self) {
        self.locked = false
    }

    /// Count pending objects in the pool
    #[inline]
    pub fn count(&self) -> usize {
        self.slab.len()
    }

}