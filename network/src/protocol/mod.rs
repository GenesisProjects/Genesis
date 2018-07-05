use common::address::Address as Account;
use common::hash::Hash;
use common::key::KeyPair;
use message::defines::*;
use peer::{PeerTable, BlockInfo};

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
                    value: account
                }
            ],
        )
    }

    //TODO: more protocols

    pub fn gossip(&self, peers: &PeerTable) -> SocketMessage {
        unimplemented!()
    }

    pub fn reject(&self, reason: String) -> SocketMessage {
        unimplemented!()
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