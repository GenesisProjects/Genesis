use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;
use common::hash::Hash;
use common::key::Signature;
use rlp::RLPSerialize;
use rlp::types::*;

use peer::*;
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

pub trait MessageCodec {
    fn encoder(&self, output: &mut [u8]);

    fn decoder(input: &[u8]) -> Self;
}

#[derive(Debug, Clone)]
pub enum SocketMessageArg {
    Int { value: u32 },
    String { value: String },
    Account { value: Account },
    Hash { value: Hash }
}


#[derive(Debug, Clone)]
pub struct SocketMessage {
    event: String,
    arg: Vec<SocketMessageArg>
}

impl SocketMessage {
    pub fn init_ping() -> Self {
        SocketMessage { event: "PING".to_string(), arg: vec![] }
    }
}


impl MessageCodec for SocketMessage {
    fn encoder(&self, output: &mut [u8]) {
        unimplemented!()
    }

    fn decoder(input: &[u8]) -> Self {
        unimplemented!()
    }
}