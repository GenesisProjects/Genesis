use chrono::prelude::*;

use common::address::Address as Account;
use common::hash::Hash;
use common::key::KeyPair;

use message::defines::*;
use nat::*;
use peer::PeerRef;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

const MAX_DELAY:i64 = 30i64;

pub trait Notify {
    /// # notify_bootstrap(&mut self, 1)
   /// **Usage**
   /// - send boostrap p2pevent
   /// ## Examples
   /// ```
   /// ```
    fn notify_bootstrap(protocol: P2PProtocol, mut peer_ref: PeerRef, table: &PeerTable);

    /// # notify_gossip(&mut self, 1)
    /// **Usage**
    /// - send gossip p2pevent
    /// ## Examples
    /// ```
    /// ```
    fn notify_gossip(protocol: P2PProtocol, mut peer_ref: PeerRef, table: &PeerTable);

    /// # heartbeat(&mut self, 1)
    /// **Usage**
    /// - send heartbeat p2pevent
    /// ## Examples
    /// ```
    /// ```
    fn heartbeat(protocol: P2PProtocol, mut peer_ref: PeerRef);
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub block_len: usize,
    pub last_block_num: usize,
    pub last_block_hash: Hash,

    pub esitmated_round: usize
}

#[derive(Debug)]
pub struct PeerTable {
    pub table: Vec<(Option<Account>, SocketInfo)>,
    pub limit: usize
}

impl Clone for PeerTable {
    fn clone(&self) -> Self {
        PeerTable {
            table: self.table.iter().map(|peer_info| peer_info.clone()).collect(),
            limit: self.limit
        }
    }
}

impl PeerTable {
    pub fn new() -> Self {
        // TODO: make limit configuable
        PeerTable {
            table: vec![],
            limit: 512
        }
    }

    pub fn new_with_hosts(hosts: Vec<(String, i32)>) -> Self {
        // TODO: make limit configuable
        PeerTable {
            table: hosts
                .into_iter()
                .map(|host| {
                    socket_info(host.0, host.1)
                })
                .filter(|socket_result| {
                    match socket_result {
                        &Ok(_) => true,
                        &Err(_) => false
                    }
                })
                .map(|socket_result| {
                    (None, socket_result.unwrap())
                })
                .collect()
            ,
            limit: 512
        }
    }

    pub fn table(&self) -> Vec<(Option<Account>, SocketInfo)> {
        self.clone().table
    }
}

/// # P2PProtocol
/// **Usage**
/// - basic protocols, implemented to generate [[SocketMessage]]
/// **Member**
/// - 1.    ***vesion***:       current client version.
/// - 2.    ***account***:      current wallet account.
/// - 3.    ***key_pair***:     client public/private keypair.
#[derive(Clone)]
pub struct P2PProtocol {
    vesion: String,
}

impl P2PProtocol {
    pub fn new() -> Self {
        //TODO: make version number configurable
        P2PProtocol {
            vesion: "0.0.0".to_string(),
        }
    }

    fn verify_version(&self, index: usize, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.version_at(index) {
            self.vesion == v
        } else {
            false
        }
    }

    fn verify_account(index: usize, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.account_at(index) {
            v.text.len() == 32
        } else {
            false
        }
    }

    fn verify_timestamp(index: usize, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.timestamp_at(index) {
            (Utc::now() - v).num_seconds() < MAX_DELAY
        } else {
            false
        }
    }

    pub fn verify(&self, msg: &SocketMessage) -> bool {
        match msg.event().as_str() {
            "BOOTSTRAP" => {
                if msg.args().len() < 4 {
                    return false;
                }

                if ! (self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)) {
                    return false;
                }

                for arg in &msg.args()[3..] {
                    match arg {
                        &SocketMessageArg::String { ref value } => {},
                        _ => { return false; }
                    }
                };
                return true;
            },
            "GOSSIP" => {
                if msg.args().len() < 4 {
                    return false;
                }

                if ! (self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)) {
                    return false;
                }

                for arg in &msg.args()[3..] {
                    match arg {
                        &SocketMessageArg::String { ref value } => {},
                        _ => { return false; }
                    }
                };
                return true;
            },
            "REJECT" => {
                if msg.args().len() != 4 {
                    return false;
                }

                if ! (self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)) {
                    return false;
                }

                match msg.string_at(3) {
                    Some(_) => true,
                    None => false
                }
            },
            "REQUEST_BLOCK_INFO" => {
                if msg.args().len() != 5 {
                    return false;
                }

                if ! (self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)) {
                    return false;
                }

                let mut ret = match msg.int_at(3) {
                    Some(_) => true,
                    None => false
                };
                ret = ret && match msg.int_at(3) {
                    Some(_) => true,
                    None => false
                };
                ret
            },
            "REQUEST_BLOCK" => {
                if msg.args().len() != 5 {
                    return false;
                }

                if ! (self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)) {
                    return false;
                }

                let mut ret = match msg.int_at(3) {
                    Some(_) => true,
                    None => false
                };
                ret = ret && match msg.int_at(3) {
                    Some(_) => true,
                    None => false
                };
                ret
            },
            _ => false
        }
    }

    pub fn bootstrap(&self, table: &PeerTable) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "BOOTSTRAP".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: Account::load().expect("Can not load account")
        } << SocketMessageArg::Timestamp {
            value: Utc::now()
        };

        for &(_, ref addr) in &table.table {
            let addr:SocketAddr = addr.0.clone();
            msg = msg << SocketMessageArg::String { value: addr.to_string() };
        }

        msg
    }

    //TODO: more protocols

    pub fn gossip(&self, table: &PeerTable) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "GOSSIP".to_string(),
            vec![]
        );
        let mut args: Vec<SocketMessageArg>  = vec![];
        msg = msg <<  SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: Account::load().expect("Can not load account")
        } << SocketMessageArg::Timestamp {
            value: Utc::now()
        };

        for host in &table.table {
            let socket_info = &host.1;
            let addr = &socket_info.0;
            let addr_str = addr.ip().to_string();
            msg = msg << SocketMessageArg::String { value: addr_str }
        }

        msg
    }

    pub fn reject(&self, reason: String) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "REJECT".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: Account::load().expect("Can not load account")
        } << SocketMessageArg::Timestamp {
            value: Utc::now()
        } << SocketMessageArg::String {
            value: reason.to_owned()
        };

        msg
    }

    pub fn request_block_info(&self,
                              self_block_len: usize,
                              self_last_hash: Hash) -> SocketMessage {

        let mut msg = SocketMessage::new(
            "REQUEST_BLOCK_INFO".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: Account::load().expect("Can not load account")
        } << SocketMessageArg::Timestamp {
            value: Utc::now()
        } << SocketMessageArg::Int {
            value: self_block_len as i32
        } << SocketMessageArg::Hash {
            value: self_last_hash.to_owned()
        };

        msg
    }

    pub fn block_info(&self,
                      block_info: &BlockInfo,
                      block_hash: Hash) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "BLOCK_INFO".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: Account::load().expect("Can not load account")
        } << SocketMessageArg::Timestamp {
            value: Utc::now()
        } << SocketMessageArg::Int {
            value: block_info.block_len as i32
        } << SocketMessageArg::Hash {
            value: block_hash
        };

        msg
    }

    pub fn heartbeat(&self) -> SocketMessage {
        SocketMessage::heartbeat()
    }

}