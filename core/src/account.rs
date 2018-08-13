use std::fmt;
use common::hash::*;
use common::address::*;
use storage::Storage;
use std::cell::{RefCell, Cell};
use rlp::RLPSerialize;
use rlp::types::*;

#[derive(Debug, Clone)]
pub struct Account {
    balance: u32,
    name: String,
    storage_root: Hash,
    storage: Storage,
    code_hash: Hash,
    address_hash: RefCell<Option<Address>>
}

impl Account {

    pub fn new(account_name: &str, storage: Storage) -> Self {
        // TODO: check account name  
        Account{
            balance: 0u32,
            name: account_name.to_string(),
            storage_root: zero_hash!(),
            storage,
            code_hash: zero_hash!(),
            address_hash: RefCell::new(None)
        }
    }

    /// return the balance associated with this account.
    pub fn balance(&self) -> u32 {
        self.balance
    }

    /// Get the storage of the account.
    pub fn storage(&self) -> Storage  {
        self.storage
    }

    /// set the value of the trie's storage with provided `key`.
    pub fn set_storage(&mut self, key: Hash, val: &[u8])  {
        self.storage.update(key, value);
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
        let account = Account::new("test", Storage::new());
        assert_eq!(account.balance, 0u32);
    }

    #[test]
    fn fmt() {
        println!("{:?}", Account::new("test", Storage::new()));
    }

}
