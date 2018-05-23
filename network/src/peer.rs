use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::{Duration, SystemTime};
use std::rc::Rc;
use std::io::*;

use mio::{Evented, Poll, PollOpt, Ready, Token};

use session::Session;
use common::address::Address;

pub type PeerRef = Rc<Peer>;

enum PeerType {
    Normal,
    Super,
}

pub struct BlockInfo {
    block_len: usize,
    last_block_num: usize,

    esitmated_cycle_num: usize
}

pub struct PeerTable {
    table: Vec<Peer>,
    limit: usize
}

pub struct Peer {
    ip_addr: SocketAddr,
    port: u16,
    peer_type: PeerType,
    connected_at: SystemTime,
    data_send: usize,
    data_received: usize,
    address: Address,
    session: Session,

    block_info: BlockInfo,
    peer_table: PeerTable
}

impl Peer {
    pub fn update_peer_table(new_peer_ref: PeerRef) {
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

