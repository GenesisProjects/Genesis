use std::fmt;
use common::hash::*;
use common::address::*;
use storage::{Storage, CHUNK, StorageCache};
use std::cell::{RefCell, Cell};
use rlp::RLPSerialize;
use rlp::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BaseAccount {
    pub nonce: u32,
    pub balance: u32,
    pub storage_root: Hash,
    pub code_hash: Hash,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Account {
    nonce: u32,
    balance: u32,
    name: String,
    storage_root: Hash,
//    storage_cache: RefCell<StorageCache>,
    storage_changes: HashMap<Hash, CHUNK>,
    code_hash: Hash,
//    address: Cell<Option<Address>>
}

impl From<BaseAccount> for Account {
    fn from(base: BaseAccount) -> Self {
        Account {
            nonce: base.nonce,
            balance: base.balance,
            storage_root: base.storage_root,
            storage_changes: HashMap::new(),
            code_hash: base.code_hash,
//            address: Cell::New(None),
            name: base.name
        }
    }
}

impl Account {
    #[cfg(test)]
    pub fn new(account_name: &str) -> Self {
        // TODO: check account name  
        Account{
            nonce: 0u32,
            balance: 0u32,
            name: account_name.to_string(),
            storage_root: zero_hash!(),
//            storage_cache: Self::clear_storage_cache(),
            storage_changes: HashMap::new(),
            code_hash: zero_hash!(),
//            address: Cell::new(None)
        }
    }



//    pub fn clear_storage_cache() -> RefCell<StorageCache> {
//        RefCell::new(StorageCache::new())
//    }

    pub fn load(addr: Address) -> Result<Self, ()> {
        unimplemented!()
    }

    /// return the balance associated with this account.
    pub fn balance(&self) -> u32 {
        self.balance
    }

    pub fn nonce(&self) -> u32 {
        self.nonce
    }

    pub fn code_hash(&self) -> Hash {
        self.code_hash.to_owned()
    }

    pub fn storage_root(&self) -> Hash {
        self.storage_root.to_owned()
    }

    /// Get the storage of the account.
//    pub fn storage(&self) -> Storage  {
//        self.storage.to_owned()
//    }

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
        Ok(
            rlp_list![
                "Account".into(),
                self.nonce.into(),
                self.balance.into(),
                String::from_utf8(self.storage_root.to_owned().to_vec()).unwrap().into(),
                String::from_utf8(self.code_hash.to_owned().to_vec()).unwrap().into(),
                self.name.to_owned().into()
            ]
        )
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        if rlp.len() != 6 {
            Err(RLPError::RLPErrorWrongNumParams)
        } else {
            let m_type: String = rlp[0].clone().into();
            if m_type != "Account".to_string() {
                Err(RLPError::RLPErrorType)
            } else {
                let storage_root_string: String = rlp[3].clone().into();
                let mut storage_root: Hash = zero_hash!();
                let code_hash_string: String = rlp[4].clone().into();
                let mut code_hash: Hash = zero_hash!();

                storage_root.copy_from_slice(storage_root_string.as_bytes());
                code_hash.copy_from_slice(code_hash_string.as_bytes());

                let base_account = BaseAccount {
                    nonce: rlp[1].clone().into(),
                    balance: rlp[2].clone().into(),
                    storage_root,
                    code_hash,
                    name: rlp[5].clone().into()
                };

                Ok(Account::from(base_account))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account() {
        let account = Account::new("test");
        assert_eq!(account.balance, 0u32);
    }

    #[test]
    fn rlp_test() {
        let a = Account::new("test");
        let b = Account::deserialize(&a.serialize().unwrap()).unwrap();
        assert_eq!(a.balance(), b.balance());
        assert_eq!(a.nonce(), b.nonce());
        assert_eq!(a.code_hash(), b.code_hash());
        assert_eq!(a.storage_root(), b.storage_root());
    }

    #[test]
    fn fmt() {
        println!("{:?}", Account::new("test"));
    }

}
