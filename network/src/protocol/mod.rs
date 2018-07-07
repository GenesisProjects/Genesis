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
        let mut msg = SocketMessage::new(
            "BOOTSTRAP".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: self.account.to_owned()
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
        let msg = SocketMessage::new(
            "REJECT".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: self.account.to_owned()
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

        let msg = SocketMessage::new(
            "REQUEST_BLOCK_INFO".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: self.account.to_owned()
        } << SocketMessageArg::Timestamp {
            value: Utc::now()
        } << SocketMessageArg::Int {
            value: self_block_len
        } << SocketMessageArg::Hash {
            value: self_last_hash.to_owned()
        };

        msg
    }

    pub fn block_info(&self, block_info: &BlockInfo) -> SocketMessage {
        let msg = SocketMessage::new(
            "BLOCK_INFO".to_string(),
            vec![]
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << SocketMessageArg::Account {
            value: self.account.to_owned()
        } << SocketMessageArg::Timestamp {
            value: Utc::now()
        } << SocketMessageArg::Int {
            value: block_info.block_len
        } << SocketMessageArg::Hash {
            value: None //Todo: Gen hash from block info
        };

        msg
    }

    //TODO: more protocols
}