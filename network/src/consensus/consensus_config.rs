use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

use common::address::Address as Account;
use config::{Config, Value, File};
use gen_core::validator::Validator;

pub trait MockConfig {
    fn mock() -> Self;
    fn mock_peer() -> Self;
}

pub struct ConsensusConfig {
    server_addr: SocketAddr,

    events_size: usize,

    update_timebase: i64,
    connect_timeout: i64,
    peer_expire: i64,

    validator_keys: Vec<Validator>
}

impl ConsensusConfig {
    pub fn load() -> Self {
        let mut config = Config::default();
        let path = Path::new("../config/application.json");
        config.merge(File::from(path))
            .expect("Could not open config");
        let consensus_config: HashMap<String, Value> = config.get_table("network.p2p")
            .expect("Could not find the p2p field in the config file");

        let port: i64 = consensus_config["port"].clone().into_int().unwrap();
        let addr = format!("127.0.0.1:{}", port);
        let server: SocketAddr = addr.parse().expect("Unable to parse socket address");

        let events_size: usize = consensus_config["events_size"].clone().into_int().unwrap() as usize;

        let update_timebase: i64 = consensus_config["update_timebase"].clone()
            .into_int().unwrap() as i64;

        let connect_timeout: i64 = consensus_config["connect_timeout"].clone()
            .into_int().unwrap() as i64;

        let peer_expire: i64 = consensus_config["peer_expire"].clone()
            .into_int().unwrap() as i64;

        let validator_keys: Vec<Validator> = consensus_config["bootstrap_peers"].clone()
            .into_array().expect("The `bootstrap_peers` is not an array")
            .into_iter().map(|value| {
            let socket_addr = value
                .into_str()
                .expect("The bootstrap peer address should be a string")
                .parse()
                .expect("Unable to parse socket address");
            Validator::new(socket_addr, Account::load().unwrap())
        }).collect();


        ConsensusConfig {
            server_addr: server,
            events_size: events_size,
            update_timebase: update_timebase,
            connect_timeout: connect_timeout,
            peer_expire: peer_expire,
            validator_keys: validator_keys
        }
    }

    pub fn server_addr(&self) -> SocketAddr {
        self.server_addr.to_owned()
    }

    pub fn events_size(&self) -> usize {
        self.events_size
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

    pub fn validator_keys(&self) -> Vec<Validator> {
        self.validator_keys.clone()
    }
}

impl MockConfig for ConsensusConfig {
    fn mock() -> Self {
        ConsensusConfig {
            server_addr:  SocketAddr::from_str("127.0.0.1:40000").unwrap(),

            events_size: 1024,

            update_timebase: 3000,
            connect_timeout: 3000,
            peer_expire: 60000,

            validator_keys: vec![
                Validator::new(SocketAddr::from_str("127.0.0.1:40001").unwrap(), Account::load().unwrap()),
                Validator::new(SocketAddr::from_str("127.0.0.1:40002").unwrap(), Account::load().unwrap()),
                Validator::new(SocketAddr::from_str("127.0.0.1:40003").unwrap(), Account::load().unwrap()),
                Validator::new(SocketAddr::from_str("127.0.0.1:40004").unwrap(), Account::load().unwrap()),
            ]
        }
    }

    fn mock_peer() -> Self {
        ConsensusConfig {
            server_addr:  SocketAddr::from_str("127.0.0.1:40001").unwrap(),

            events_size: 1024,

            update_timebase: 3000,
            connect_timeout: 3000,
            peer_expire: 60000,

            validator_keys: vec![
                Validator::new(SocketAddr::from_str("127.0.0.1:40000").unwrap(), Account::load().unwrap()),
                Validator::new(SocketAddr::from_str("127.0.0.1:40002").unwrap(), Account::load().unwrap()),
                Validator::new(SocketAddr::from_str("127.0.0.1:40003").unwrap(), Account::load().unwrap()),
                Validator::new(SocketAddr::from_str("127.0.0.1:40004").unwrap(), Account::load().unwrap()),
            ]
        }
    }
}