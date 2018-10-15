use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use common::address::Address as Account;
use config::{Config, Value, File};

pub trait MockConfig {
    fn mock() -> Self;
    fn mock_peer() -> Self;
}

pub struct P2PConfig {
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

impl P2PConfig {
    pub fn load() -> Self {
        let mut config = Config::default();
        let path = Path::new("../config/application.json");
        config.merge(File::from(path))
            .expect("Could not open config");
        let p2p_config: HashMap<String, Value> = config.get_table("network.p2p")
            .expect("Could not find the p2p field in the config file");

        let port: i64 = p2p_config["port"].clone().into_int().unwrap();
        let addr = format!("127.0.0.1:{}", port);
        let server: SocketAddr = addr.parse().expect("Unable to parse socket address");

        let events_size: usize = p2p_config["events_size"].clone().into_int().unwrap() as usize;

        let max_allowed_peers: usize = p2p_config["max_allowed_peers"].clone()
            .into_int().unwrap() as usize;

        let max_blocklist_size: usize = p2p_config["max_blocklist_size"].clone()
            .into_int().unwrap() as usize;

        let max_waitinglist_size: usize = p2p_config["max_waitinglist_size"].clone()
            .into_int().unwrap() as usize;

        let min_required_peer: usize = p2p_config["min_required_peer"].clone()
            .into_int().unwrap() as usize;

        let update_timebase: i64 = p2p_config["update_timebase"].clone()
            .into_int().unwrap() as i64;

        let connect_timeout: i64 = p2p_config["connect_timeout"].clone()
            .into_int().unwrap() as i64;

        let peer_expire: i64 = p2p_config["peer_expire"].clone()
            .into_int().unwrap() as i64;

        let bootstrap_peers: Vec<(Option<Account>, SocketAddr)> = p2p_config["bootstrap_peers"].clone()
            .into_array().expect("The `bootstrap_peers` is not an array")
            .into_iter().map(|value| {
            let socket_addr = value
                .into_str()
                .expect("The bootstrap peer address should be a string")
                .parse()
                .expect("Unable to parse socket address");
            (None, socket_addr)
        }).collect();


        P2PConfig {
            server_addr: server,
            events_size: events_size,
            max_allowed_peers: max_allowed_peers,
            max_blocklist_size: max_blocklist_size,
            max_waitinglist_size: max_waitinglist_size,
            min_required_peer: min_required_peer,
            update_timebase: update_timebase,
            connect_timeout: connect_timeout,
            peer_expire: peer_expire,
            bootstrap_peers: bootstrap_peers
        }
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

impl MockConfig for P2PConfig {
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