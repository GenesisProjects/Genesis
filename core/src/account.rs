use rlp::RLPSerialize;
use common::hash::*;
use super::mpt::trie::*;

use std::io::*;

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

    /// Get the value of the trie's storage with provided `key`.
    pub fn storage_val(&self, key: Hash) -> Vec<u8>  {
        unimplemented!()
    }
}