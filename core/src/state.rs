use account::Account;
use storage::Storage;
use std::collections::HashMap;
use common::address::Address;
use std::cell::RefCell;
use std::error::Error;
use mpt::trie::Trie;

pub struct State {
    accounts: RefCell<HashMap<String, Account>>,
//    storage_map: RefCell<HashMap<String, Storage>>
//    trie:
}

impl State {
    pub fn new() -> State {
        State {
            accounts: RefCell::new(HashMap::new()),
//            storage_map: RefCell::new(HashMap::new())
        }
    }

    pub fn get_account(name: String) -> &Account {
        unimplemented!()
    }

    pub fn get_account_by_address(addr: Address) -> &Account {
        unimplemented!()
    }

    pub fn commit(&mut self) -> Result<(), Error>{
        let mut accounts = self.accounts.borrow_mut();

        for (&account_name, ref mut account) in accounts.iter_mut() {
            account.commit_storage();
        }

        // TODO: get trie from memory or db, then insert or delete

        Ok(())
    }
}