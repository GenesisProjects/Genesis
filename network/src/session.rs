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

/// # TaskContext
/// **usage**
/// - record the current transmission task infos
/// **Member**
/// - 1. ***data_send***:        data send to the peer session
/// - 2. ***data_recv***:        data receive from the peer session
/// - 3. ***size_expected***:    only using in the transmission mode, total data size expected
/// ## Examples
/// ```
/// ```
pub struct TaskContext {
    data_send: usize,
    data_recv: usize,
    size_expected: usize
}

impl TaskContext {
    /// # new(1)
    /// **usage**
    /// - init a new task context with expected size
    /// **Parameters**
    /// - 1. ***usize(expected)***:  the current task size
    /// **return**
    /// - ***Self***: new object
    /// ## Examples
    /// ```
    /// ```
    pub fn new(expected: usize) -> Self {
        TaskContext {
            data_send: 0,
            data_recv: 0,
            size_expected: 0,
        }
    }

    /// # reset(self, 1)
    /// **usage**
    /// - reset the current context with expected size
    /// **Parameters**
    /// - 1. ***usize(expected)***:  the current task size
    /// ## Examples
    /// ```
    /// ```
    pub fn reset(&mut self, expected: usize) {
        self.data_send = 0;
        self.data_recv = 0;
        self.size_expected = 0;
    }
}

/// # SessionStatus: Debug + Clone
/// **usage**
/// - enum of the session status
/// **enum**
/// - 1.    ***Init***:
/// - 2.    ***Idle***:
/// - 3.    ***Transmission***:
/// - 4.    ***ConnectionReject***:
/// - 5.    ***WaitGosship***:
/// - 6.    ***WaitBlockInfo***:
/// - 7.    ***WaitSyncInfo***:
/// - 8.    ***WaitTransmission***:
/// - 9.    ***WaitingBlockInfoRequest***:
/// - 10.   ***WaitTransmissionRequest***:
/// - 11.   ***WaitSyncInfoRequest***:
/// - 12.   ***WaitTransmissionAccept***:
/// ## Examples
/// ```
/// ```
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
                    context: TaskContext::new(0usize),
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
        self.socket.receive_msgs().and_then(|msgs| {
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

# [cfg(test)]
mod tests {
    # [test]
    fn test_session() {}
}