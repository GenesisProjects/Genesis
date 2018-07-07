use common::address::Address as Account;
use common::hash::Hash;
use common::key::KeyPair;
use message::defines::*;
use nat::*;
use chrono::prelude::*;

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

    pub fn verify(&self, msg: &SocketMessage) -> bool {
        // TODO:
        match msg.event().as_str() {
            _ => false
        }
    }

    pub fn bootstrap(&self) -> SocketMessage {
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

}