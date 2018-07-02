use message::defines::*;
use common::address::Address as Account;

pub struct P2PProtocol {
    vesion: String,
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

/*
#[derive(Debug, Clone)]
pub enum RejectReason {

}

#[derive(Debug, Clone)]
pub enum P2PMessage {
    /// Invoked when bootstrap to peers
    Bootstrap(SocketAddr, Account),
    /// Invoked when peers bootstrap to us if we accept
    Accept(SocketAddr, Account, BlockInfo, PeerTable),
    /// Invoked when peers bootstrap to us if we reject
    Reject(SocketAddr, Account, RejectReason),
}


#[derive(Debug, Clone)]
pub enum ChainMessage {
    /// pre-request for a peer to provide a new chain - start position, current chain length
    ChainSyncInit(Account, Hash, u64),
    /// answer for the pre-request - peer_id, fork point, increased chain length
    ChainSyncInitAnswer(Account, Option<Hash>, u64),
    /// request for download a chain - peer_id, fork point, increased chain length
    ChainSyncRequest(Account, Hash, u64),
    /// resource ready - peer_id, fork point, increased chain length, file size, checksum
    ChainSyncReady(Account, Hash, u64, usize, u32),
    /// blockchain downloaded - peer_id, fork point, increased chain length, file size, checksum
    ChainSyncReceived(Account, Hash, u64, usize, u32),
}

*/
