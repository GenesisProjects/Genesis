use chrono::*;

use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use std::io::*;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;

use common::address::*;
use frame::*;
use socket::*;

enum SessionStatus {
    Connected,
    Disconnected,
    Transmitting,
}

pub struct Session {
    socket: PeerSocket,
    status: SessionStatus,
    cur_task: Task,
    account: Address,
    addr: SocketAddr,
    created: DateTime<Utc>,
    data_send: usize,
    data_recv: usize,
    frame_send: usize,
    frame_recv: usize,
}

impl Session {
    pub fn connect(addr: &SocketAddr) -> Self {

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