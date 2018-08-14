use std::fmt;
use common::hash::*;
use common::address::*;
use storage::{Storage, CHUNK, StorageCache, AccountStorage};
use std::cell::{RefCell, Cell};
use rlp::RLPSerialize;
use rlp::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Account {
    balance: u32,
    name: String,
    storage_root: Hash,
    storage_cache: RefCell<StorageCache>,
    storage_changes: HashMap<Hash, CHUNK>,
    code_hash: Hash,
    address: Cell<Option<Address>>
}

impl Account {
    pub fn new(account_name: &str, storage: Storage) -> Self {
        // TODO: check account name  
        Account{
            balance: 0u32,
            name: account_name.to_string(),
            storage_root: zero_hash!(),
            storage_cache: Self::clear_storage_cache(),
            storage_changes: HashMap::new(),
            code_hash: zero_hash!(),
            address: Cell::new(None)
        }
    }

    pub fn clear_storage_cache() -> RefCell<StorageCache> {
        RefCell::new(StorageCache::new())
    }

    /// return the balance associated with this account.
    pub fn balance(&self) -> u32 {
        self.balance
    }

    /// Get the storage of the account.
    pub fn storage(&self) -> Storage  {
        self.storage.to_owned()
    }

    /// set the value of the trie's storage with provided `key`.
    pub fn set_storage(&mut self, key: Hash, val: CHUNK)  {
        self.storage_changes.insert(key, val);
//        self.storage.update(key, val);
    }

    /// update storage changes and save to db
    pub fn commit_storage(&mut self) {
        unimplemented!()
    }

    /// Increase account balance.
    pub fn add_balance(&mut self, x: u32) {
        self.balance = self.balance + x;
    }

    /// Decrease account balance.
    /// TODO: when balance is less than x
    pub fn sub_balance(&mut self, x: u32) {
        assert!(self.balance >= x);
        self.balance = self.balance - x;
    }
}

impl RLPSerialize for Account {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account() {
        //let account = Account::new("test", Storage::new());
        //assert_eq!(account.balance, 0u32);
    }

    #[test]
    fn fmt() {
        //println!("{:?}", Account::new("test", Storage::new()));
    }

}
