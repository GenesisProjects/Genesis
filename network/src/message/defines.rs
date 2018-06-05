use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;
use common::hash::Hash;

use peer::*;

enum RejectReason {

}

#[derive(Debug, Clone)]
pub enum P2PMessage {
    /// Invoked when bootstrap to peers
    Bootstrap(SocketAddr, Account),
    /// Invoked when peers bootstrap to us
    Register(SocketAddr, Account, BlockInfo, PeerTable),
    /// Invoked when peers bootstrap to us
    Reject(SocketAddr, Account, ),
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