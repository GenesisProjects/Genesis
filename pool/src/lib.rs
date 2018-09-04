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

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::mem;

pub enum PoolError {
    DBError(String),
    Duplicate(Hash),
    NonceError,
    Locked,
}

pub trait Poolable {
    /// Priority in queue
    fn score(&self) -> ScoreRecord;
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
    fn score(&self) -> ScoreRecord {
        unimplemented!()
    }

    fn hash(&self) -> Hash {
        unimplemented!()
    }

    fn nonce(&self) -> u64 {
        unimplemented!()
    }

    fn verify(&self) -> Result<(), PoolError> {
        unimplemented!()
    }

    fn account(&self) -> Address {
        unimplemented!()
    }
}

/// Assign each element with a score, so it will sorted in priority queue.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScoreRecord {
    pub id: Hash,
    pub score: usize
}

impl Ord for ScoreRecord {
    fn cmp(&self, other: &ScoreRecord) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for ScoreRecord {
    fn partial_cmp(&self, other: &ScoreRecord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Object pool, which uses slab allocator to manage memory
/// The object stored must be uniform type and implement [ScoreGen] trait
/// Should notify all channels after added a new object
pub struct Pool<T> where T: Poolable {
    name: String,
    channels: Vec<String>,
    slab: Slab<T>,
    slab_key_map: HashMap<Hash, usize>,
    priority_queue: BinaryHeap<ScoreRecord>,
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
            priority_queue: BinaryHeap::new(),
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
    pub fn notify_new_tx_recieved(&self) {
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
            let score = obj.score();
            let slab_key = self.slab.insert(obj);
            self.slab_key_map.insert(hash, slab_key);
            self.priority_queue.push(score);
            self.notify_new_tx_recieved();
            Ok(())
        }
    }

    /// Pop out a enqueued object
    /// High score object will first out
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self.priority_queue.pop() {
            Some(record) => {
                let hash = record.id;
                let (_, slab_key) = self.slab_key_map.remove_entry(&hash).unwrap();
                let obj = self.slab.remove(slab_key);
                Some(obj)
            },
            None => None
        }
    }

    /// Clear the pending object pool
    #[inline]
    fn clear(&mut self) -> Result<(), PoolError> {
        self.priority_queue.clear();
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