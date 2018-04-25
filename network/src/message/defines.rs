use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address;
use common::hash::Hash;

#[derive(Debug, Clone)]
pub enum P2PMessage {
    /// Invoked when a bootstrap peer connects to us
    Accept(SocketAddr, Address),
    /// Invoked when we bootstrap to a new peer.
    Connect(SocketAddr, Address),
    /// Invoked when we failed to connect to all bootstrap contacts.
    Failed,
    /// Invoked when we are ready to listen for incomming connection. Contains
    /// the listening port.
    ListenerStarted(u16),
    /// Invoked when listener failed to start.
    ListenerFailed,
    /// Invoked when a peer disconnects or can no longer be contacted.
    LostPeer(Address),
    /// Invoked when a new message is received. Passes the message.
    NewMessage(Address, Vec<u8>),
    /// Invoked when trying to sending a too large data.
    WriteMsgSizeProhibitive(Address, Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum ChainMessage {
    /// pre-request for a peer to provide a new chain - start position, current chain length
    ChainSyncInit(Address, Hash, u64),
    /// answer for the pre-request - peer_id, fork point, increased chain length
    ChainSyncInitAnswer(Address, Option<Hash>, u64),
    /// request for download a chain - peer_id, fork point, increased chain length
    ChainSyncRequest(Address, Hash, u64),
    /// resource ready - peer_id, fork point, increased chain length, file size, checksum
    ChainSyncReady(Address, Hash, u64, usize, u32),
    /// blockchain downloaded - peer_id, fork point, increased chain length, file size, checksum
    ChainSyncReceived(Address, Hash, u64, usize, u32),
}