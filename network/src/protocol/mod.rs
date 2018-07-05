use message::defines::*;

use common::address::Address as Account;
use common::key::KeyPair;

/// # P2PController
/// **Usage**
/// - basic protocols, generate [[SocketMessage]]
/// **Member**
/// - 1.    ***vesion***:              current wallet account.
pub struct P2PProtocol {
    vesion: String,
    account: Account,
    key_pair: KeyPair
}

impl P2PProtocol {
    pub fn bootstrap(&self, account: Account) -> SocketMessage {
        SocketMessage::new(
            "BOOTSTRAP".to_string(),
            vec![
                SocketMessageArg::Vesion {
                    value: self.vesion.to_owned()
                },
                SocketMessageArg::Account {
                    value: account
                }
            ]
        )
    }

    //TODO: more protocols
}
