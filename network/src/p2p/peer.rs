use std::cell::RefCell;
use std::net::SocketAddr;
use std::rc::{Rc, Weak};
use std::io::*;
use std::result::Result as SerdeResult;

use common::address::Address as Account;
use nat::*;
use socket::message::message_handler::*;

use super::protocol::*;
use super::session::*;
use super::session_state::*;

use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::net::TcpStream;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub type PeerRef = Rc<RefCell<Peer>>;
pub type WeakPeerRef = Weak<RefCell<Peer>>;

pub const INIT_CREDIT: u32 = 800u32;
pub const INIT_TTL: u32 = 255u32;

#[derive(Clone,Debug)]
enum PeerType {
    Normal,
    Super,
    Unknown
}

#[derive(Debug)]
pub struct Peer {
    bootstraped: bool,
    ip_addr: SocketAddr,
    peer_type: PeerType,
    account: Option<Account>,
    credit: u32,
    token: Option<Token>,
    ttl: u32,

    pub session: Session
}

impl Peer {
    #[inline]
    pub fn new(socket: TcpStream, addr: &SocketAddr) -> Self {
        Peer {
            bootstraped: false,
            ip_addr: addr.clone(),
            peer_type: PeerType::Unknown,
            account: None,
            credit: INIT_CREDIT,
            token: None,
            ttl: INIT_TTL,

            session: Session::new(socket, addr).add_handler(),
        }
    }

    #[inline]
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        Session::connect(addr).and_then(|session| {
            Ok(Peer {
                bootstraped: false,
                ip_addr: addr.clone(),
                peer_type: PeerType::Unknown,
                account: None,
                credit: INIT_CREDIT,
                token: None,
                ttl: INIT_TTL,

                session: session.add_handler()
            })
        })
    }

    #[inline]
    pub fn peer_should_kill(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    pub fn table(&self) -> PeerTable {
        self.session.table()
    }

    #[inline]
    pub fn bootstraped(&self) -> bool {
        self.bootstraped
    }

    #[inline]
    pub fn set_bootstraped(&mut self) {
        self.bootstraped = true;
    }

    #[inline]
    pub fn status(&self) -> SessionStatus {
        self.session.status()
    }

    #[inline]
    pub fn peer_table(&self) -> Vec<(Option<Account>, SocketInfo)> {
        self.table().table
    }

    #[inline]
    pub fn account(&self) -> Option<Account> {
        self.account.clone()
    }

    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.ip_addr.clone()
    }

    #[inline]
    pub fn credit(&self) -> u32 {
        self.credit
    }

    #[inline]
    pub fn set_token(&mut self, token: Token) {
        self.token = Some(token);
        self.session.set_token(token);
    }

    #[inline]
    pub fn process(&mut self, name: String) -> Result<()> {
        match self.session.process(name) {
            Ok(penalty) => {
                self.ttl = INIT_TTL;
                if penalty <= self.credit {
                    self.credit -= penalty;
                } else {
                    self.credit = 0;
                }
                Ok(())
            },
            Err(e) => {
                if self.ttl > 0 {
                    self.ttl -= 1;
                } else {
                    self.session.set_status(SessionStatus::Abort);
                }
                Err(e)
            }
        }
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

