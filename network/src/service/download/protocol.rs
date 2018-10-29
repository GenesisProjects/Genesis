use chrono::prelude::*;
use common::address::Address;
use common::hash::Hash;
use gen_core::transaction::Transaction;
use nat::*;
use serde::ser::*;
use serde::de::*;
use serde_json;
use serde_json::Result;
use socket::message::defines::*;
use std::net::SocketAddr;

const MAX_DELAY: i64 = 30i64;

pub const PEER_SYNC_STR: &'static str = "PEER_SYNC_STR";
pub const ASK_BLOCK_INFO_STR: &'static str = "ASK_BLOCK_INFO";
pub const BLOCK_INFO_STR: &'static str = "BLOCK_INFO";
pub const ASK_BLOCK_STR: &'static str = "ASK_BLOCK";
pub const BLOCK_STR: &'static str = "BLOCK";
pub const ASK_TX_STR: &'static str = "ASK_TX";
pub const TX_STR: &'static str = "TX_STR";
pub const ASK_ACCOUNT_STR: &'static str = "ASK_ACCOUNT";
pub const ACCOUNT_STR: &'static str = "ACCOUNT";

#[derive(Debug, Clone)]
pub struct PeerInfo {
    account: Address,
    cur_height: u64,
    tail_hash: Hash
}

impl PeerInfo {
    pub fn account(&self) -> Address {
        self.account.clone()
    }

    pub fn cur_height(&self) -> u64 {
        self.cur_height
    }

    pub fn tail_hash(&self) -> Hash {
        self.tail_hash.clone()
    }

    pub fn new(
        account: Address,
        cur_height: u64,
        tail_hash: Hash
    ) -> PeerInfo {
        PeerInfo {
            account: account,
            cur_height: cur_height,
            tail_hash: tail_hash
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncProtocol {
    vesion: String,
}

impl SyncProtocol {
    pub fn new(vesion: &str) -> Self {
        SyncProtocol {
            vesion: vesion.to_string()
        }
    }

    fn verify_version(&self, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.version_at(0) {
           v == self.vesion
        } else {
            false
        }
    }

    fn verify_timestamp(&self, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.timestamp_at(1) {
            (Utc::now() - v).num_seconds() < MAX_DELAY
        } else {
            false
        }
    }

    fn verify_msg_header(&self, msg: &SocketMessage) -> bool {
        self.verify_version(msg) && self.verify_timestamp(msg)
    }

    fn add_msg_header(&self, msg: SocketMessage) -> SocketMessage {
        msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << Utc::now().into()
    }

    /// Build peer sync message
    pub fn sync(&self, info: &PeerInfo) -> SocketMessage {
        let mut msg = SocketMessage::new(
            PEER_SYNC_STR.to_string(),
            vec![],
            vec![],
        );
        msg = self.add_msg_header(msg);
        msg = msg << info.account().into() << info.cur_height().into() << info.tail_hash().into();
        msg
    }

    /// Parse peer sync message
    pub fn parse_sync(&self, msg: &SocketMessage) -> Option<PeerInfo> {
        if !self.verify_msg_header(msg) {
            return None
        } else {
            match (msg.account_at(2), msg.uint_at(3), msg.hash_at(4)) {
                (Some(a), Some(u), Some(h)) => {
                    Some(PeerInfo::new(a, u, h))
                }
                _ => None
            }
        }
    }

    /// Build find ancestor message.
    pub fn find_ancestor(&self, common_block: Hash) -> SocketMessage {
        let mut msg = SocketMessage::new(
            PEER_SYNC_STR.to_string(),
            vec![],
            vec![],
        );
        msg = self.add_msg_header(msg);
        msg = msg << info.account().into() << info.cur_height().into() << info.tail_hash().into();
        msg
    }

    /// Parse peer sync message
    pub fn parse_find_ancestor(&self, msg: &SocketMessage) -> Option<PeerInfo> {
        if !self.verify_msg_header(msg) {
            return None
        } else {
            match (msg.account_at(2), msg.uint_at(3), msg.hash_at(4)) {
                (Some(a), Some(u), Some(h)) => {
                    Some(PeerInfo::new(a, u, h))
                }
                _ => None
            }
        }
    }
}