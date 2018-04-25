use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::{Duration, SystemTime};

use peer_manager::PeerManager;
use session::Session;

use common::address::Address;

enum PeerType {
    Normal,
    Super,
}

enum PeerStatus {
    Actived,
    Disabled,
    Disconnected,
}

pub struct Peer {
    socket: SocketAddr,
    port: u16,
    peer_type: PeerType,
    connected_at: SystemTime,
    data_send: usize,
    data_received: usize,
    address: Address,
    status: PeerStatus,
    session: Session,
}

impl Peer {
    pub fn is_on_black_list(manager: &PeerManager) -> bool {
        unimplemented!()
    }
}

