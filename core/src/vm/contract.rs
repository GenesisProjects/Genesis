use std::borrow::Cow;
use std::collections::HashMap;

use chrono::*;

use common::address::Address;
use rlp::RLPSerialize;
use rlp::types::{RLPError, RLP};
use account::Account;

pub type AccountRef<'a> =  Cow<'a, Account>;

pub struct Contract<'a> {
    groups: HashMap<String, Vec<Address>>,
    create: DateTime<Utc>,
    expire: DateTime<Utc>,
    account: AccountRef<'a>
}

impl<'a> RLPSerialize for Contract<'a> {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

impl<'a> Contract<'a> {
    #[inline]
    pub fn get_account(&self) -> AccountRef {
        self.account
    }
}