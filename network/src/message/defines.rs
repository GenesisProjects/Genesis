use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;
use common::hash::Hash;
use rlp::RLPSerialize;
use rlp::types::*;

use peer::*;
use frame::*;

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

impl RLPSerialize for P2PMessage {
    fn serialize(&self) -> Result<RLP, RLPError> {
        match self {
            &P2PMessage::Bootstrap(ref addr, ref account) => {
                Ok(RLP::RLPList { list: vec![account.serialize().unwrap_or(RLP::RLPItem { value: "".into() })] })
            },
            _ => Err(RLPError::RLPErrorUnknown)
        }
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

impl FrameSerialize for P2PMessage {
    fn serialize(&self) -> Vec<Frame> {
        unimplemented!()
    }
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