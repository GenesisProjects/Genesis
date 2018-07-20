use std::borrow::Cow;
use std::collections::HashMap;

use chrono::*;
use wasmi::Signature;

use common::address::Address;
use rlp::RLPSerialize;
use rlp::types::{RLPError, RLP};

pub struct Contract {
    create: DateTime<Utc>,
    expire: DateTime<Utc>,

    abi_table: Vec<Signature>
}

impl RLPSerialize for Contract {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}