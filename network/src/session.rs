use chrono::*;

use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use std::io::*;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;

use common::address::Address as Account;
use pool_manager::SHARED_POOL_MANAGER;
use socket::*;
use message::defines::*;

#[derive(Debug, Clone)]
pub enum SessionStatus {
    Init,
    RequestConnection,      // Client Only
    EstablishConnection,    // Server Only
    Rejected,               // Server Only

    Connected,
    Disconnected,
    Idle,
    WaitReceiving,
    Receiving,
    WaitTransmitting,
    Transmitting
}

pub struct TaskContext {
    data_send: usize,
    data_recv: usize,
    request_count: usize,
    response_count: usize
}

impl TaskContext {
    pub fn new() -> Self {
        TaskContext {
            data_send: 0,
            data_recv: 0,
            request_count: 0,
            response_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.data_send = 0;
        self.data_recv = 0;
        self.request_count = 0;
        self.response_count = 0;
    }
}

pub struct Session {
    socket: PeerSocket,
    status: SessionStatus,
    addr: SocketAddr,
    created: DateTime<Utc>,
    context: TaskContext
}

impl Session {
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        match PeerSocket::connect(addr) {
            Ok(r) => {
                Ok(Session {
                    socket: r,
                    status: SessionStatus::Init,
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

    pub fn status(&self) -> SessionStatus {
        self.status.clone()
    }

    fn process_events(&mut self) {
        self.socket.receive().and_then(|msgs| {
            for msg_ref in &msgs {
                self.process_single_event(msg_ref);
            }
            Ok(msgs)
        });
    }

    fn process_single_event(&mut self, msg: &SocketMessage) {
        let event = msg.get_event();
        let event = event.as_str();
        let args = msg.get_args();
        match event {
            //TODO: process logic
            _ => {}
        }
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