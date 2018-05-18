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

pub struct TaskContext {
    data_send: usize,
    data_recv: usize,
    frame_send: usize,
    frame_recv: usize,
    cur_task: Task,
    last_frame: Option<FrameRef>
}

impl TaskContext {
    pub fn new() -> Self {
        TaskContext {
            data_send: 0,
            data_recv: 0,
            frame_send: 0,
            frame_recv: 0,
            cur_task: Task::Idle,
            last_frame: None
        }
    }
}

pub struct Session {
    socket: PeerSocket,
    status: SessionStatus,
    account: Account,
    addr: SocketAddr,
    created: DateTime<Utc>,
    context: TaskContext
}

impl Session {
    pub fn connect(addr: &SocketAddr, account: Account) -> Result<Self> {
        match PeerSocket::connect(addr) {
            Ok(r) => {
                Ok(Session {
                    socket: r,
                    status: SessionStatus::Init,
                    account: account.clone(),
                    addr: addr.clone(),
                    created: Utc::now(),
                    context: TaskContext::new()
                })
            },
            Err(e) => Err(e)
        }
    }

    pub fn disconnect(addr: &SocketAddr) -> Self {
        unimplemented!()
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