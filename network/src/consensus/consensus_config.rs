use std::net::SocketAddr;
use std::str::FromStr;
use common::address::Address as Account;

pub trait MockConfig {
    fn mock() -> Self;
    fn mock_peer() -> Self;
}

pub struct ConsensusConfig {
    server_addr: SocketAddr,

    events_size: usize,

    max_allowed_peers: usize,
    max_blocklist_size: usize,
    max_waitinglist_size: usize,
    min_required_peer: usize,

    update_timebase: i64,
    connect_timeout: i64,
    peer_expire: i64,

    bootstrap_peers: Vec<(Option<Account>, SocketAddr)>
}

impl ConsensusConfig {
    pub fn load() -> Self {
        unimplemented!()
    }

    pub fn server_addr(&self) -> SocketAddr {
        self.server_addr.to_owned()
    }

    pub fn events_size(&self) -> usize {
        self.events_size
    }

    pub fn max_allowed_peers(&self) -> usize {
        self.max_allowed_peers
    }

    pub fn max_blocklist_size(&self) -> usize {
        self.max_blocklist_size
    }

    pub fn max_waitinglist_size(&self) -> usize {
        self.max_waitinglist_size
    }

    pub fn min_required_peer(&self) -> usize {
        self.min_required_peer
    }

    pub fn update_timebase(&self) -> i64 {
        self.update_timebase
    }

    pub fn connect_timeout(&self) -> i64 {
        self.update_timebase
    }

    pub fn peer_expire(&self) -> i64 {
        self.peer_expire
    }

    pub fn bootstrap_peers(&self) -> Vec<(Option<Account>, SocketAddr)> {
        self.bootstrap_peers.clone()
    }
}

impl MockConfig for ConsensusConfig {
    fn mock() -> Self {
        P2PConfig {
            server_addr:  SocketAddr::from_str("127.0.0.1:40000").unwrap(),

            events_size: 1024,

            max_allowed_peers: 512,
            max_blocklist_size: 1024,
            max_waitinglist_size: 1024,
            min_required_peer: 5,

            update_timebase: 3000,
            connect_timeout: 3000,
            peer_expire: 60000,

            bootstrap_peers: vec![
                (None, SocketAddr::from_str("127.0.0.1:40001").unwrap()),
                (None, SocketAddr::from_str("127.0.0.1:40002").unwrap()),
                (None, SocketAddr::from_str("127.0.0.1:40003").unwrap()),
                (None, SocketAddr::from_str("127.0.0.1:40004").unwrap())
            ]
        }
    }

    fn mock_peer() -> Self {
        P2PConfig {
            server_addr:  SocketAddr::from_str("127.0.0.1:40001").unwrap(),

            events_size: 1024,

            max_allowed_peers: 512,
            max_blocklist_size: 1024,
            max_waitinglist_size: 1024,
            min_required_peer: 5,

            update_timebase: 3000,
            connect_timeout: 3000,
            peer_expire: 60000,

            bootstrap_peers: vec![
                (None, SocketAddr::from_str("127.0.0.1:40000").unwrap()),
                (None, SocketAddr::from_str("127.0.0.1:40002").unwrap()),
                (None, SocketAddr::from_str("127.0.0.1:40003").unwrap()),
                (None, SocketAddr::from_str("127.0.0.1:40004").unwrap())
            ]
        }
    }
}