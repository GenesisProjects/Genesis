use chrono::*;

use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use std::io::*;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;

use common::address::Address as Account;
use common::hash::Hash;
use gen_message::{MESSAGE_CENTER, Message};

use message::defines::*;
use message::protocol::*;
use nat::*;
use pool_manager::SHARED_POOL_MANAGER;
use p2p_controller::CHANNEL_NAME;
use socket::*;

pub const CMD_ERR_PENALTY: u32 = 100u32;
pub const DATA_TRANS_ERR_PENALTY: u32 = 200u32;

/// # TaskContext
/// **Usage**
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
    /// **Usage**
    /// - init a new task context with expected size
    /// **Parameters**
    /// - 1. ***usize(expected)***:  the current task size
    /// **Return**
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
    /// **Usage**
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
/// **Usage**
/// - enum of the session status
/// **Enum**
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
    Abort,

    // client reserved
    WaitGosship,
    WaitBlockInfo,
    WaitSyncInfo,
    WaitTransmission,

    // server reserved
    WaitBlockInfoRequest,
    WaitTransmissionRequest,
    WaitSyncInfoRequest,
    WaitTransmissionAccept
}

/// # SessionMode：Debug + Clone
/// **Usage**
/// - record the current transmission task infos
/// **Enum**
/// - 1. ***Transmission***:    session will only accept binary data
/// - 2. ***Command***:         session will only accept control signals
/// ## Examples
/// ```
/// ```
#[derive(Debug, Clone)]
pub enum SessionMode {
    Transmission,
    Command
}

/// # Session
/// **Usage**
/// - wrapper of socket, manage p2p protocol state machine
/// **Member**
/// - 1. ***socket***:      instance of [PeerSocket]
/// - 2. ***status***:      instance of [SessionStatus]
/// - 3. ***addr***:        copy of socket
/// - 4. ***created***:     created date in utc timezone
/// - 5. ***context***:     instance of [TaskContext]
/// - 6. ***connected***:   true if communication session has been established
/// - 6. ***mode***:        instance of [SessionMode]
pub struct Session {
    token: Option<Token>,
    socket: PeerSocket,
    status: SessionStatus,
    addr: SocketAddr,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    context: TaskContext,
    connected: bool,
    mode: SessionMode,
    protocol: P2PProtocol,

    table: PeerTable,
    block_info: Option<BlockInfo>
}

impl Session {
    /// # new(2)
    /// **Usage**
    /// - accept the connection and init a new session by incoming socket,
    /// - init the default session status to [[SessionStatus::Init]]
    /// - init the default session mode to [[SessionMode::Command]]
    /// - set the 'created' to the current time.
    /// **Parameters**
    /// - 1. ***TcpStream(socket)***: tcp socket instance
    /// - 1. ***&SocketAddr(addr)***: the socket address we try to connect
    /// **Return**
    /// - 1. ***Self***
    /// ## Examples
    /// ```
    /// ```
    pub fn new(socket: TcpStream, addr: &SocketAddr) -> Self {
        Session {
            token: None,
            socket: PeerSocket::new(socket),
            status: SessionStatus::Init,
            addr: addr.clone(),
            created: Utc::now(),
            updated: Utc::now(),
            context: TaskContext::new(0usize),
            connected: false,
            mode: SessionMode::Command,
            protocol: P2PProtocol::new(),

            table: PeerTable::new(),
            block_info: None
        }
    }

    /// # connect(1)
    /// **Usage**
    /// - setup a socket connection,
    /// - init the default session status to [[SessionStatus::Init]]
    /// - init the default session mode to [[SessionMode::Command]]
    /// - set the 'created' to the current time.
    /// **Parameters**
    /// - 1. ***&SocketAddr(addr)***: the socket address we try to connect
    /// **Return**
    /// - 1. ***Result<Self>***
    /// ## Examples
    /// ```
    /// ```
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        match PeerSocket::connect(addr) {
            Ok(r) => {
                Ok(Session {
                    token: None,
                    socket: r,
                    status: SessionStatus::Init,
                    addr: addr.clone(),
                    created: Utc::now(),
                    updated: Utc::now(),
                    context: TaskContext::new(0usize),
                    connected: false,
                    mode: SessionMode::Command,
                    protocol: P2PProtocol::new(),

                    table: PeerTable::new(),
                    block_info: None
                })
            },
            Err(e) => Err(e)
        }
    }

    /// # disconnect(&mut self, 0)
    /// **Usage**
    /// - abort current socket
    /// **Return**
    /// - 1. ***Result<()>***
    /// ## Examples
    /// ```
    /// ```
    pub fn disconnect(&mut self) {
        self.status = SessionStatus::Abort;
    }

    #[inline]
    pub fn status(&self) -> SessionStatus {
        self.status.clone()
    }

    #[inline]
    pub fn block_info(&self) -> Option<BlockInfo> {
        self.block_info.clone()
    }

    #[inline]
    pub fn table(&self) -> PeerTable {
        self.table.clone()
    }

    #[inline]
    pub fn connected(&self) -> bool {
        self.connected
    }

    #[inline]
    pub fn milliseconds_from_last_update(&self) -> i64 {
        (Utc::now() - self.updated).num_milliseconds()
    }

    #[inline]
    pub fn set_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    /// # process(&mut self)
   /// **Usage**
   /// - process the incoming byte stream
   /// - if the session mode is [[SessionMode::Command]], then call process_events
   /// - if the session mode is [[SessionMode::Transmisson]], then call process_data
   /// ## Examples
   /// ```
   /// ```
    #[inline]
    pub fn process(&mut self) -> u32 {
        self.updated = Utc::now();
        match self.mode {
            SessionMode::Command => self.process_events(),
            SessionMode::Transmission => self.process_data()
        }
    }

    #[inline]
    pub fn send_data(&mut self, data: &[u8]) -> Result<()> {
        self.socket.send_data(data)
    }

    #[inline]
    pub fn send_event(&mut self, msg: SocketMessage) -> Result<()> {
        self.socket.send_msg(msg)
    }

    #[inline]
    fn toggle_mode(&mut self, mode: SessionMode) {
        self.mode = mode;
    }

    #[inline]
    fn set_connect(&mut self, connected: bool) {
        self.connected = true;
    }

    fn process_data(&mut self) -> u32 {
        unimplemented!()
    }

    fn process_events(&mut self) -> u32 {
        let mut err_count: usize = 0usize;
        self.socket.receive_msgs().and_then(|msgs| {
            for msg_ref in &msgs {
                if !self.process_single_event(msg_ref) {
                    err_count += 1;
                }
            }
            Ok(msgs)
        });
        (err_count as u32) * CMD_ERR_PENALTY
    }

    fn process_single_event(&mut self, msg: &SocketMessage) -> bool {
        let event = msg.event();
        let event = event.as_str();
        let args = &msg.args();
        match event {
            "BOOTSTRAP" => {
                if !self.protocol.verify(&msg) {
                    false
                } else {
                    match self.status {
                        SessionStatus::Init => {
                            let slice = &args[3 .. ];
                            let mut hosts: Vec<String> = vec![];
                            for arg in slice {
                                match arg {
                                    &SocketMessageArg::String { ref value } => {
                                        //TODO: make port configurable
                                        hosts.push(value.clone())
                                    }
                                    _ => ()
                                };
                            }
                            self.table = PeerTable::new_with_hosts(hosts);
                            self.status = SessionStatus::WaitGosship;
                            // notify controller send gossip
                            if let Some(token) = self.token.clone() {
                                MESSAGE_CENTER.lock().unwrap().send(
                                    &CHANNEL_NAME.to_string(),
                                    Message::new(token.0 as u16, "gossip".to_string())
                                );
                            }

                            true
                        },
                        _ => {
                            // print cmd output here
                            unimplemented!();
                            false
                        }
                    }
                }
            },
            "GOSSIP" => {
                if !self.protocol.verify(&msg) {
                    false
                } else {
                    match self.status {
                        SessionStatus::Init => {
                            let slice = &args[3 .. ];
                            let mut hosts: Vec<String> = vec![];
                            for arg in slice {
                                match arg {
                                    &SocketMessageArg::String { ref value } => {
                                        //TODO: make port configurable
                                        hosts.push(value.clone())
                                    }
                                    _ => ()
                                };
                            }
                            self.table = PeerTable::new_with_hosts(hosts);
                            self.status = SessionStatus::WaitBlockInfoRequest;
                            true
                        },
                        _ => {
                            // print cmd output here
                            unimplemented!();
                            false
                        }
                    }
                }
            },
            "REJECT" => {
                if !self.protocol.verify(&msg) {
                    false
                } else {
                    match &args[3] {
                        &SocketMessageArg::String { ref value } => {
                            // print cmd output here
                            unimplemented!()
                        },
                        _ => {
                            return false;
                        }
                    }
                    match self.status {
                        SessionStatus::Init | SessionStatus::WaitBlockInfoRequest => {
                            self.status = SessionStatus::ConnectionReject;
                            true
                        },
                        _ => false
                    }
                }
            },
            _ => false
        }
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

impl Drop for Session {
    fn drop(&mut self) {
        unimplemented!()
    }
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_session() {}
}