use rlp::RLPSerialize;

use common::hash::*;

use std::io::*;

use storage::Storage;

use super::mpt::trie::*;

#[derive(Debug, Clone)]
pub struct Account {
    balance: u32,
    storage_root: Hash
}

impl Account {

    pub fn new() -> Self {
        Account{
            balance: 0u32,
            storage_root: zero_hash!()
        }
    }

    /// return the balance associated with this account.
    pub fn balance(&self) -> u32 { self.balance }

    /// Get the storage of the account
    pub fn storage(&self) -> Storage  {
        unimplemented!()
    }
}