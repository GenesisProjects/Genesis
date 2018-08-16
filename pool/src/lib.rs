pub extern crate common;
pub extern crate gen_message;
pub extern crate slab;

use ::common::hash::*;
use ::common::observe::*;
use ::slab::Slab;
use gen_message::*;

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::mem;

pub enum PoolError {
    Duplicate(Hash),
    Validation(String),
    Locked,
}

pub trait Poolable {
    /// Priority in queue
    fn score(&self) -> ScoreRecord;
    /// Unique hash id
    fn hash(&self) -> Hash;
    /// The round counter
    fn round(&self) -> usize;
    /// Verify if the object is valid
    fn verify(&self) -> Result<(), PoolError>;
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
    next_round: usize,

    pending_slab: Slab<T>,
    pending_slab_key_map: HashMap<Hash, usize>,
    pending_priority_queue: BinaryHeap<ScoreRecord>,

    cur_slab: Slab<T>,
    cur_slab_key_map: HashMap<Hash, usize>,
    cur_priority_queue: BinaryHeap<ScoreRecord>,

    locked: bool
}

impl<T> Pool<T> where T: Poolable {
    /// Init pool with capacity
    pub fn new(name: String, size: usize, next_round: usize) -> Self {
        Pool {
            name: name,
            channels: vec![],
            next_round: next_round,

            pending_slab: Slab::with_capacity(size),
            pending_slab_key_map: HashMap::new(),
            pending_priority_queue: BinaryHeap::new(),

            cur_slab: Slab::with_capacity(size),
            cur_slab_key_map: HashMap::new(),
            cur_priority_queue: BinaryHeap::new(),

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
        self.pending_slab_key_map.get(&hash).is_some()
    }

    fn check(&self, obj: &T) -> Result<(), PoolError> {
        if obj.round() != self.next_round {
            Err(PoolError::Validation("The input object is not at it's round".into()))
        } else {
            obj.verify()
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
            let slab_key = self.pending_slab.insert(obj);
            self.pending_slab_key_map.insert(hash, slab_key);
            self.pending_priority_queue.push(score);
            self.notify_new_tx_recieved();
            Ok(())
        }
    }

    /// Pop out a enqueued object
    /// High score object will first out
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self.cur_priority_queue.pop() {
            Some(record) => {
                let hash = record.id;
                let (_, slab_key) = self.cur_slab_key_map.remove_entry(&hash).unwrap();
                let obj = self.cur_slab.remove(slab_key);
                Some(obj)
            },
            None => None
        }
    }

    /// swap the current object pool with pending object pool
    #[inline]
    fn swap(&mut self) -> Result<(), PoolError> {
        mem::swap(&mut self.cur_slab, &mut self.pending_slab);
        mem::swap(&mut self.cur_slab_key_map, &mut self.pending_slab_key_map);
        mem::swap(&mut self.cur_priority_queue, &mut self.pending_priority_queue);
        self.clear_pending();
        Ok(())
    }

    /// Clear the object pool
    #[inline]
    fn clear_cur(&mut self) -> Result<(), PoolError> {
        self.cur_priority_queue.clear();
        self.cur_slab.clear();
        self.cur_slab_key_map.clear();
        Ok(())
    }

    /// Clear the pending object pool
    #[inline]
    fn clear_pending(&mut self) -> Result<(), PoolError> {
        self.pending_priority_queue.clear();
        self.pending_slab.clear();
        self.pending_slab_key_map.clear();
        Ok(())
    }


    /// Swap pending and current object pools and prepare for next round
    #[inline]
    pub fn next_round(&mut self) {
        self.swap();
        self.next_round += 1;
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
        self.pending_slab.len()
    }

    /// Count current objects in the pool
    #[inline]
    pub fn count_cur(&self) -> usize {
        self.cur_slab.len()
    }
}