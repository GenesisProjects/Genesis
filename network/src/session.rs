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

pub struct TaskContext {
    data_send: usize,
    data_recv: usize,
    size_expected: usize
}

impl TaskContext {
    pub fn new() -> Self {
        TaskContext {
            data_send: 0,
            data_recv: 0,
            size_expected: 0,
        }
    }

    pub fn reset(&mut self) {
        self.data_send = 0;
        self.data_recv = 0;
        self.size_expected = 0;
    }
}

#[derive(Debug, Clone)]
pub enum SessionStatus {
    Init,
    Idle,
    Transmission,
    ConnectionReject,

    // client reserved
    WaitGosship,
    WaitBlockInfo,
    WaitSyncInfo,
    WaitTransmission,

    // server reserved
    WaitingBlockInfoRequest,
    WaitTransmissionRequest,
    WaitSyncInfoRequest,
    WaitTransmissionAccept
}

#[derive(Debug, Clone)]
pub enum SessionMode {
    Transmission,
    Command
}

pub struct Session {
    socket: PeerSocket,
    status: SessionStatus,
    addr: SocketAddr,
    created: DateTime<Utc>,
    context: TaskContext,
    connected: bool,
    mode: SessionMode
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
                    context: TaskContext::new(),
                    connected: false,
                    mode: SessionMode::Command
                })
            },
            Err(e) => Err(e)
        }
    }

    pub fn disconnect(addr: &SocketAddr) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn status(&self) -> SessionStatus {
        self.status.clone()
    }

    #[inline]
    pub fn process(&mut self) {
        match self.mode {
            SessionMode::Command => self.process_events(),
            SessionMode::Transmission => self.process_data()
        }
    }

    #[inline]
    fn toggle_mode(&mut self, mode: SessionMode) {
        self.mode = mode;
    }

    #[inline]
    fn set_connect(&mut self, connected: bool) {
        self.connected = true;
    }

    fn process_data(&mut self) {
        unimplemented!()
    }

    fn process_events(&mut self) {
        self.socket.receive().and_then(|msgs| {
            for msg_ref in &msgs {
                self.process_single_event(msg_ref);
            }
            Ok(msgs)
        });
    }

    fn process_single_event(&mut self, msg: &SocketMessage) -> bool {
        let event = msg.get_event();
        let event = event.as_str();
        let args = &msg.get_args();
        match event {
            //TODO: process logic
            "BOOTSTRAP" => {
                unimplemented!()
            },
            _ => unimplemented!()
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