use std::ops::*;

use common::address::Address as Account;
use common::hash::{Hash, HASH_LEN};
use chrono::*;
use rust_base58::{ToBase58, FromBase58};

use serde::ser::*;
use serde::de::*;

static DATE_FMT: &'static str = "%Y-%m-%d-%H-%M-%S-%f";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketMessageArg {
    Int { value: i32 },
    String { value: String },
    Account { value: String },
    Hash { value: Hash },
    Vesion { value: String },
    Timestamp { value: String },
    Unknown
}

impl From<Account> for SocketMessageArg {
    fn from(account: Account) -> Self {
        SocketMessageArg::Account {
            value: account.text
        }
    }
}

impl From<DateTime<Utc>> for SocketMessageArg {
    fn from(date: DateTime<Utc>) -> Self {
        SocketMessageArg::Timestamp {
            value: date.format(DATE_FMT).to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketMessage {
    event: String,
    arg: Vec<SocketMessageArg>,
    payload: Vec<u8>
}

impl Shl<SocketMessageArg> for SocketMessage {
    type Output = Self;

    fn shl(self, arg: SocketMessageArg) -> SocketMessage {
        let mut new_self = self.clone();
        new_self.arg.push(arg);
        new_self
    }
}

impl SocketMessage {
    pub fn new(event: String, arg: Vec<SocketMessageArg>, payload: Vec<u8>) -> Self {
        SocketMessage {
            event: event,
            arg: arg,
            payload: payload
        }
    }

    pub fn int_at(&self, index: usize) -> Option<i32> {
        match self.arg[index] {
            SocketMessageArg::Int { value } => Some(value),
            _ => None
        }
    }

    pub fn string_at(&self, index: usize) -> Option<String> {
        match &self.arg[index] {
            &SocketMessageArg::String { ref value } => Some(value.clone()),
            _ => None
        }
    }

    pub fn account_at(&self, index: usize) -> Option<Account> {
        match &self.arg[index] {
            &SocketMessageArg::Account { ref value } => {
                if value.len() == HASH_LEN {
                    Some(Account{ text: value.clone() })
                } else {
                    None
                }
            },
            _ => None
        }
    }

    pub fn hash_at(&self, index: usize) -> Option<Hash> {
        match self.arg[index] {
            SocketMessageArg::Hash { value } => Some(value),
            _ => None
        }
    }

    pub fn version_at(&self, index: usize) -> Option<String> {
        match &self.arg[index] {
            &SocketMessageArg::Vesion { ref value } => Some(value.clone()),
            _ => None
        }
    }

    pub fn timestamp_at(&self, index: usize) -> Option<DateTime<Utc>> {
        match self.arg[index] {
            SocketMessageArg::Timestamp { ref value } => {
                match Utc.datetime_from_str(
                    value.as_str(),
                    DATE_FMT
                ) {
                    Ok(r) => Some(r),
                    _ => None
                }
            },
            _ => None
        }
    }

    pub fn event(&self) -> String {
        self.event.to_owned()
    }

    pub fn args(&self) -> Vec<SocketMessageArg> {
        self.arg.to_owned()
    }

    pub fn heartbeat() -> Self {
        SocketMessage {
            event: "HEARTBEAT".to_string(),
            arg: vec![],
            payload: vec![]
        }
    }

    pub fn exception(reason: &str) -> Self {
        SocketMessage {
            event: "EXCEPTION".to_string(),
            arg: vec![SocketMessageArg::String { value: reason.to_string() }],
            payload: vec![]
        }
    }
}