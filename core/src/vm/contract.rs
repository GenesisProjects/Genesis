use std::borrow::Cow;
use std::collections::HashMap;

use chrono::*;

use common::address::Address;
use rlp::RLPSerialize;
use rlp::types::{RLPError, RLP};

pub struct Contract {
    groups: HashMap<String, Vec<Address>>,
    create: DateTime<Utc>,
    expire: DateTime<Utc>
}

impl RLPSerialize for Contract {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}