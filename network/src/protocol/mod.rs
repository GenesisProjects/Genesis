use common::address::Address as Account;
use common::hash::Hash;
use common::key::KeyPair;
use message::defines::*;
use peer::{PeerTable, BlockInfo};
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
                SocketMessage::Timestamp {
                    value: DateTime<Utc> = Utc::now()
                }
            ],
        )
    }

    //TODO: more protocols

    pub fn gossip(&self, peers: &PeerTable) -> SocketMessage {
        SocketMessage::new(
            "GOSSIP".to_string(),
            vec![
                SocketMessageArg::Vesion {
                    value: self.vesion.to_owned()
                },
                SocketMessageArg::Account {
                    value: self.account.to_owned()
                },
                SocketMessage::Timestamp {
                    value: DateTime<Utc> = Utc::now()
                },
                SocketMessage::String {
                    value: {
                        let mut peers_table = peers.values().map(|peer| {
                            peer.peer_table()
                        }).fold(Vec<String>::new(), |mut init, ref mut table: Vec<(Option<Account>, SocketInfo)>| {
                            let addr_table = table.iter().map(|(ref account, (ref addr, ref port))| {
                                addr.to_string()
                            }).collect();

                            init.append(addr_table);
                            init
                        });
                        peers_table.to_string()
                    }
                }
            ],
        )
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
                SocketMessage::Timestamp {
                    value: DateTime<Utc> = Utc::now()
                },
                SocketMessage::String {
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