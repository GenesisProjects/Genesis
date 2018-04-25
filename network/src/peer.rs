use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::{Duration, SystemTime};

use peer_manager::PeerManager;
use session::Session;

enum PeerType {
    Normal,
    Producer,
}

pub struct Peer {
    socket: SocketAddr,
    port: u16,
    peer_type: PeerType,
    connected_at: SystemTime,
    data_send: usize,
    data_received: usize,
    session: Session,
}

impl Peer {
    pub fn is_on_black_list(manager: &PeerManager) -> bool {
        unimplemented!()
    }
}

