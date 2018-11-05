extern crate common;
extern crate gen_core;
#[macro_use]
extern crate gen_message;
extern crate gen_processor;
extern crate slab;

use common::address::*;
use common::hash::*;
use gen_processor::*;
use gen_core::blockchain::chain_service;
use gen_core::transaction::Transaction;
use gen_message::{MESSAGE_CENTER, Message, defines::pool::*};
use slab::Slab;
use std::any::Any;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

const TIME_SPAN: u64 = 100;

pub enum PoolError {
    DBError(String),
    Duplicate(Hash),
    NonceError,
    TransactionError(String),
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
        if self.check() {
            Ok(())
        } else {
            Err(PoolError::TransactionError("transaction check failed".to_string()))
        }
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
    nonce_map: HashMap<Address, u64>,
    chain_service: chain_service::ChainService,
    locked: bool
}

impl<T> Pool<T> where T: Poolable {
    /// Init pool with capacity
    pub fn new(name: String, slab_size: usize) -> Self {
        Pool {
            name: name,
            channels: vec![],
            slab: Slab::with_capacity(slab_size),
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
            .for_each(|ch| {
                notify!(ch.to_string(), Message::new("new_tx".to_string(), 0, vec![]));
            });
    }

    /// If the object exist in pending pool
    #[inline]
    pub fn exist(&self, obj: &T) -> bool {
        let hash = obj.hash();
        self.slab_key_map.get(&hash).is_some()
    }

    /// Check if the nonce is correct
    fn verify_nonce(&mut self, obj: &T) -> Result<bool, chain_service::DBError> {
        self.get_account_nonce(obj.account()).and_then(|nonce| {
            Ok(nonce == obj.nonce())
        })
    }

    /// Get the account nonce from the last block in block chain
    fn get_account_nonce(&mut self, account_addr: Address) -> Result<u64, chain_service::DBError> {
        if let Some(n) = self.nonce_map.get(&account_addr) {
            return Ok(*n);
        }
        let result = self.chain_service.get_last_block_account_nonce(account_addr.clone());
        match result {
            Ok(r) => {
                self.nonce_map.insert(account_addr, r);
                Ok(r)
            },
            Err(e) => Err(e)
        }
    }

    /// Drop the nonce cache
    pub fn drop_nonce_cache(&mut self) {
       self.nonce_map = HashMap::new()
    }

    /// Check if the transaction is valid
    fn check(&mut self, obj: &T) -> Result<(), PoolError> {
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

pub struct TransactionPoolController {
    pool: Pool<Transaction>,
    status: ThreadStatus,
    recv: Option<Receiver<Message>>
}

impl Processor for TransactionPoolController {
    fn name(&self) -> String {
        "TransactionPoolManager".to_string()
    }

    fn description(&self) -> String {
        "".to_string()
    }

    fn status(&self) -> ThreadStatus {
        self.status
    }

    fn set_status(&mut self, status: ThreadStatus) {
        self.status = status;
    }

    fn receiver(&self) -> &Option<Receiver<Message>> {
        &self.recv
    }

    fn set_receiver(&mut self, recv: Receiver<Message>) {
        self.recv = Some(recv);
    }

    fn handle_msg(&mut self, msg: Message) {
        match msg.msg().as_ref() {
            CLEAN_NONCE_CACHE => self.pool.drop_nonce_cache(),
            _ => (),
        }
    }

    fn exec(&mut self) -> bool {
        // do nothing here
        true
    }

    fn pre_exec(&mut self) -> bool {
        // do nothing here
        true
    }

    fn time_span(&self) -> u64 {
        TIME_SPAN
    }
}

impl TransactionPoolController {
    pub fn create(slab_size: usize, name: String) -> ContextRef<Self> {
        let pool: Pool<Transaction> = Pool::new(name, slab_size);
        let controller = TransactionPoolController {
            pool: pool,
            status: ThreadStatus::Stop,
            recv: None
        };
        controller.launch()
    }

    pub fn pool(&self) -> &Pool<Transaction> {
        &self.pool
    }

    pub fn mut_pool(&mut self) -> &mut Pool<Transaction> {
        &mut self.pool
    }
}
