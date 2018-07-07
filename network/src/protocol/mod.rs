use common::address::Address as Account;
use common::hash::Hash;
use common::key::KeyPair;
use message::defines::*;
use nat::*;
use session::{PeerTable, BlockInfo};
use chrono::prelude::*;

/// # P2PController
/// **Usage**
/// - basic protocols, implemented to generate [[SocketMessage]]
/// **Member**
/// - 1.    ***vesion***:       current client version.
/// - 2.    ***account***:      current wallet account.
/// - 3.    ***key_pair***:     client public/private keypair.
pub struct P2PProtocol {
    vesion: String,
    account: Account,
    key_pair: KeyPair,
}

impl P2PProtocol {
    pub fn bootstrap(&self) -> SocketMessage {
        SocketMessage::new(
            "BOOTSTRAP".to_string(),
            vec![
                SocketMessageArg::Vesion {
                    value: self.vesion.to_owned()
                },
                SocketMessageArg::Account {
                    value: self.account.to_owned()
                },
                SocketMessageArg::Timestamp {
                    value: Utc::now()
                }
            ],
        )
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
            value: self.account.to_owned()
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
        SocketMessage::new(
            "REJECT".to_string(),
            vec![
                SocketMessageArg::Vesion {
                    value: self.vesion.to_owned()
                },
                SocketMessageArg::Account {
                    value: self.account.to_owned()
                },
                SocketMessageArg::Timestamp {
                    value: Utc::now()
                },
                SocketMessageArg::String {
                    value: reason.to_owned()
                },
            ],
        )
    }

    pub fn request_block_info(&self,
                              reason: String,
                              self_block_len: usize,
                              self_last_hash: Hash) -> SocketMessage {
        unimplemented!()
    }

    pub fn block_info(&self, block_info: &BlockInfo) -> SocketMessage {
        unimplemented!()
    }

    //TODO: more protocols
}