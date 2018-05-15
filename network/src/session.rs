use chrono::*;

use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use std::io::*;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;

use common::address::Address as Account;
use frame::*;
use socket::*;

enum SessionStatus {
    Init,
    Connected,
    Disconnected,
    Transmitting,
}

pub struct Session {
    socket: PeerSocket,
    status: SessionStatus,
    cur_task: Option<Task>,
    account: Account,
    addr: SocketAddr,
    created: DateTime<Utc>,
    data_send: usize,
    data_recv: usize,
    frame_send: usize,
    frame_recv: usize,
}

impl Session {
    pub fn connect(addr: &SocketAddr, account: Account) -> Result<Self> {
        match PeerSocket::connect(addr) {
            Ok(r) => {
                Ok(Session {
                    socket: r,
                    status: SessionStatus::Init,
                    cur_task: None,
                    account: account.clone(),
                    addr: addr.clone(),
                    created: Utc::now(),
                    data_send: 0,
                    data_recv: 0,
                    frame_send: 0,
                    frame_recv: 0
                })
            },
            e@Err(_) => e
        }
    }

    pub fn disconnect(addr: &SocketAddr) -> Self {

    }

}

impl Evented for Session {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.socket.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.socket.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> Result<()> {
        self.socket.deregister(poll)
    }
}