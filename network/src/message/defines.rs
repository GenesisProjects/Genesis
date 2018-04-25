use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address;

#[derive(Debug, Clone)]
pub enum P2PMessage {
    /// Invoked when a bootstrap peer connects to us
    BootstrapAccept(SocketAddr, Address),
    /// Invoked when we bootstrap to a new peer.
    BootstrapConnect(SocketAddr, Address),
    /// Invoked when we failed to connect to all bootstrap contacts.
    BootstrapFailed,
    /// Invoked when we are ready to listen for incomming connection. Contains
    /// the listening port.
    ListenerStarted(u16),
    /// Invoked when listener failed to start.
    ListenerFailed,
    /// Invoked when connection to a new peer has been established.
    ConnectSuccess(Address),
    /// Invoked when connection to a new peer has failed.
    ConnectFailure(Address),
    /// Invoked when a peer disconnects or can no longer be contacted.
    LostPeer(Address),
    /// Invoked when a new message is received. Passes the message.
    NewMessage(Address, Vec<u8>),
    /// Invoked when trying to sending a too large data.
    WriteMsgSizeProhibitive(Address, Vec<u8>),
}