//! Socket message components definition.

use std::ops::*;

use common::address::Address as Account;
use common::hash::{Hash, HASH_LEN};
use chrono::*;
use rust_base58::{ToBase58, FromBase58};
use serde::ser::*;
use serde::de::*;
use serde_json;
use serde_json::Result;
use std::net::SocketAddr;

static DATE_FMT: &'static str = "%Y-%m-%d-%H-%M-%S-%f";

pub const EXCEPTION_STR: &'static str = "EXCEPTION";
pub const HEARTBEAT_STR: &'static str = "HEARTBEAT";
pub const DISCOVERY_STR: &'static str = "DISCOVERY";
pub const PEER_INFO_STR: &'static str = "PEER_INFO";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketMessageArg {
    Int { value: i64 },
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

/// Socket message.
/// The following example shows how to build a message.
///
/// ```ignore
/// let mut msg = SocketMessage::new(
///     "TEST".to_string(),
///     vec![],
///     vec![],
/// );
/// let args: Vec<SocketMessageArg> = vec![];
///
/// // Message builder
/// msg = msg << Account::load().expect("Can not load account").into() << Utc::now().into() << SocketMessageArg::Int {
///     value: self_block_len as i32
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketMessage {
    /// Event name
    event: String,
    /// Argument list
    arg: Vec<SocketMessageArg>,
    /// External data payload
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

    pub fn clone_payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    /// Set message event name.
    pub fn set_event(&mut self, event: String) {
        self.event = event;
    }

    /// Fetch integer argument with the index.
    /// Return `None` if type dismatch.
    pub fn int_at(&self, index: usize) -> Option<i64> {
        match self.arg[index] {
            SocketMessageArg::Int { value } => Some(value),
            _ => None
        }
    }

    /// Fetch string argument with the index.
    /// Return `None` if type dismatch.
    pub fn string_at(&self, index: usize) -> Option<String> {
        match &self.arg[index] {
            &SocketMessageArg::String { ref value } => Some(value.clone()),
            _ => None
        }
    }

    /// Fetch account address argument with the index.
    /// Return `None` if type dismatch.
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

    /// Fetch sha256 hash argument with the index.
    /// Return `None` if type dismatch.
    pub fn hash_at(&self, index: usize) -> Option<Hash> {
        match self.arg[index] {
            SocketMessageArg::Hash { value } => Some(value),
            _ => None
        }
    }

    /// Fetch `Version` argument with the index.
    /// Return `None` if type dismatch.
    pub fn version_at(&self, index: usize) -> Option<String> {
        match &self.arg[index] {
            &SocketMessageArg::Vesion { ref value } => Some(value.clone()),
            _ => None
        }
    }

    /// Fetch Utc timestamp argument with the index.
    /// Return `None` if type dismatch.
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

    /// Return socket message event name
    pub fn event(&self) -> String {
        self.event.to_owned()
    }

    /// Return list of `SocketMessageArg`
    pub fn args(&self) -> Vec<SocketMessageArg> {
        self.arg.to_owned()
    }

    /// Build a heartbeat message.
    pub fn heartbeat() -> Self {
        SocketMessage {
            event: String::from(HEARTBEAT_STR),
            arg: vec![],
            payload: vec![]
        }
    }

    /// Detect if the message is exception
    pub fn is_heartbeat(&self) -> bool {
        self.event == String::from(HEARTBEAT_STR)
    }

    /// Build a exception message.
    pub fn exception(reason: &str) -> Self {
        SocketMessage {
            event: String::from(EXCEPTION_STR),
            arg: vec![SocketMessageArg::String { value: reason.to_string() }],
            payload: vec![]
        }
    }

    /// Detect if the message is exception
    pub fn is_exception(&self) -> bool {
        self.event == String::from(EXCEPTION_STR)
    }

    /// Detect if the message is exception
    pub fn exception_msg(&self) -> Option<String> {
        self.string_at(0)
    }

    /// Build a discovery message
    pub fn discovery() -> Self {
        SocketMessage {
            event: String::from(DISCOVERY_STR),
            arg: vec![],
            payload: vec![]
        }
    }

    /// Detect if the message is discovery
    pub fn is_discovery(&self) -> bool {
        self.event == String::from(DISCOVERY_STR)
    }

    /// Build a peer info message
    pub fn peer_info(peer_addrs: Vec<SocketAddr>) -> Self {
        // serialize the socket message
        let mut new_data = serde_json::to_string(&peer_addrs).unwrap().into_bytes();
        SocketMessage {
            event: String::from(PEER_INFO_STR),
            arg: vec![],
            payload: new_data
        }
    }

    /// Build a peer info message
    pub fn parse_peer_info(&self) -> Result<Vec<SocketAddr>> {
        // clone the payload
        let mut palyload = self.clone_payload();
        let json_str = unsafe { String::from_utf8_unchecked(palyload) };
        let peer_addrs: Result<Vec<SocketAddr>> = serde_json::from_str(&json_str);
        peer_addrs
    }
}