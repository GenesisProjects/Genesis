use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::{Duration, SystemTime};
use std::rc::{Rc, Weak};
use std::io::*;
use std::result::Result as SerdeResult;

use common::address::Address as Account;
use session::*;
use nat::SocketInfo;

use mio::{Evented, Poll, PollOpt, Ready, Token};
use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub type PeerRef = Rc<Peer>;
pub type WeakPeerRef = Weak<Peer>;

#[derive(Clone,Debug)]
enum PeerType {
    Normal,
    Super,
    Unknown
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
    block_len: usize,
    last_block_num: usize,

    esitmated_cycle_num: usize
}

#[derive(Debug)]
pub struct PeerTable {
    table: Vec<(Account, SocketInfo)>,
    limit: usize
}

impl Clone for PeerTable {
    fn clone(&self) -> Self {
        PeerTable {
            table: self.table.iter().map(|peer_info| peer_info.clone()).collect(),
            limit: self.limit
        }
    }
}

impl PeerTable {
    pub fn new() -> Self {
        // TODO: make limit configuable
        PeerTable {
            table: vec![],
            limit: 512
        }
    }

    pub fn table(&self) -> Vec<(Account, SocketInfo)> {
        self.clone().table
    }
}

pub struct Peer {
    ip_addr: SocketAddr,
    peer_type: PeerType,
    account: Option<Account>,
    session: Session,

    block_info: Option<BlockInfo>,
    peer_table: PeerTable
}

impl Peer {
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        Session::connect(addr).and_then(|session| {
            Ok(Peer {
                ip_addr: addr.clone(),
                peer_type: PeerType::Unknown,
                account: None,
                session: session,

                block_info: None,
                peer_table: PeerTable::new()
            })
        })
    }

    pub fn update(&mut self, block_info: &BlockInfo) {
        self.block_info = Some(block_info.clone());
    }

    pub fn update_peer_table(new_peer_ref: PeerRef) {
        unimplemented!()
    }

    pub fn status(&self) -> SessionStatus {
        self.session.status()
    }

    pub fn peer_table(&self) -> Vec<(Account, SocketInfo)> {
        self.peer_table.clone().table
    }

    pub fn account(&self) -> Option<Account> {
        self.account.clone()
    }

    pub fn addr(&self) -> SocketAddr {
        self.ip_addr.clone()
    }
}

impl Serialize for Peer {
    fn serialize<S>(&self, serializer: S) -> SerdeResult<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer {
        unimplemented!()
    }
}

impl Deserialize for Peer {
    fn deserialize<D>(deserializer: D) -> SerdeResult<Self, <D as Deserializer>::Error> where D: Deserializer {
        unimplemented!()
    }
}

impl Evented for Peer {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.session.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.session.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> Result<()> {
        self.session.deregister(poll)
    }
}

