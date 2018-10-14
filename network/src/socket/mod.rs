//! TCP Socket Wrapper
//!
//! Genesis use this crate to send peer to peer messages.
//! All I/O operations are **non-blocking**
//!
//! # Examples
//! ```ignore
//! // Show general usage of PeerSocket
//! use gen_network::socket::PeerSocket;
//! use gen_network::socket::message::defines::*;
//! use mio::tcp::TcpStream;
//!
//! // Connect to another peer
//! let mut socket = PeerSocket::connect("127.0.0.1:30001".into()).unwrap();
//!
//! // Send a socket message
//! socket.send_msg(SocketMessage::heartbeat());
//!
//! // Receive socket messages from another peer
//! match self.socket.receive_msgs() {
//!     // Get messages successful
//!     Ok(msgs) => {
//!         for msg in msgs {
//!             println!(msg);
//!         }
//!     },
//!     // Socket is not available right now
//!     Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
//!         println!("Socket is not ready anymore, please try again in the next tick");
//!     },
//!     // IO exception
//!     Err(e) => {
//!        panic!("Exception: {:?}", e);
//!     }
//! }
//! ```
//! ```ignore
//! // Integrated with event loop
//! use gen_network::socket::PeerSocket;
//! use gen_network::socket::message::defines::*;
//! use gen_network::eventloop::*;
//!
//! // Connect to another peer
//! let mut socket = PeerSocket::connect("127.0.0.1:30001".into()).unwrap();
//!
//! // Register into an event loop
//! let mut eventloop: NetworkEventLoop<PeerSocket> = NetworkEventLoop::new(1024);
//! let token = eventloop.register_peer(&socket).unwrap();
//!
//! // Event loop fetch events in next tick
//! eventloop.next_tick();
//!
//! // Try to receive new socket messages if the socket is ready
//! for event in &(eventloop.events) {
//!     if event.token() == token {
//!         match socket.receive_msgs() {
//!             // Get messages successful
//!             Ok(msgs) => {
//!                 for msg in msgs {
//!                     println!(msg);
//!                 }
//!             },
//!             // Socket is not available right now
//!             Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
//!                 println!("Socket is not ready anymore, please try again in the next tick");
//!             },
//!             // IO exception
//!             Err(e) => {
//!                 panic!("Exception: {:?}", e);
//!             }
//!         }
//!     }
//! }
//! ```
//!
pub mod message;

use chrono::*;
use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use serde_json;
use std::io::{Error, ErrorKind, Read, Write};
use std::io::Result as STDResult;
use std::net::{Shutdown, SocketAddr};

use self::message::{defines::*, SocketMessageHeader, MSG_HEADER_LEN, MSG_PAYLOAD_LEN};

pub const MAX_WRITE_BUFF_SIZE: usize = 1024 * 1024 * 1024;
pub const MAX_READ_BUFF_SIZE: usize = 1024 * 1024 * 1024;

/// The max mio data window size.
pub const MIO_WINDOW_SIZE: usize = 1024;

/// The max mio data window size.
pub const KEEP_ALIVE_MS: u32 = 100;

/// Init retry times.
pub const INIT_RETRY_TIMES: u32 = 255u32;

#[derive(Debug, Clone)]
pub struct PeerSocketStat {
    data_send: usize,
    data_recv: usize,
    last_send_time: DateTime<Utc>,
    last_recv_time: DateTime<Utc>,
    send_speed: f64,
    recv_speed: f64
}

impl PeerSocketStat {
    pub fn new() -> Self {
        PeerSocketStat {
            data_send: 0,
            data_recv: 0,
            last_send_time: Utc::now(),
            last_recv_time: Utc::now(),
            send_speed: 0.0,
            recv_speed: 0.0
        }
    }

    pub fn data_send(&self) -> usize {
        self.data_send
    }

    pub fn data_recv(&self) -> usize {
        self.data_recv
    }

    pub fn send_speed(&self) -> f64 {
        self.send_speed
    }

    pub fn recv_speed(&self) -> f64 {
        self.recv_speed
    }

    pub fn notify_send(&mut self, size: usize) {
        self.data_send += size;
        let time_now = Utc::now();
        let duration = time_now - self.last_send_time;
        if duration.num_milliseconds() > 0 {
            self.send_speed = (size as f64) / (duration.num_milliseconds() as f64);
        }
        self.last_send_time = time_now;
    }

    pub fn notify_recv(&mut self, size: usize) {
        self.data_recv += size;
        let time_now = Utc::now();
        let duration = time_now - self.last_recv_time;
        if duration.num_milliseconds() > 0 {
            self.recv_speed = (size as f64) / (duration.num_milliseconds() as f64);
        }
        self.last_recv_time = time_now;
    }
}

/// A non-blocking TCP socket between peer and peer.
/// The data send by socket is sealed within [SocketMessage](message/defines/SocketMessage.t.html)..
/// PeerSocket only support `ipv4` address now.
#[derive(Debug)]
pub struct PeerSocket {
    token: Option<Token>,
    stream: TcpStream,
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
    stat: PeerSocketStat,
    r_prep: bool,
    w_prep: bool,
    retry_times: u32,
    alive: bool
}

impl PeerSocket {
    /// Init a peer socket with mio [TcpStream](../../mio/tcp/TcpStream.t.html)
    #[inline]
    pub fn new(socket: TcpStream) -> Self {
        let result = PeerSocket {
            token: None,
            stream: socket,
            read_buffer: vec![],
            write_buffer: vec![],
            stat: PeerSocketStat::new(),
            r_prep: false,
            w_prep: false,
            retry_times: INIT_RETRY_TIMES,
            alive: true
        };
        result.setup_socket();
        result
    }

    /// Get the socket address.
    /// The address string should be unique for different socket.
    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }

    /// Establish a connection to the a socket address directly.
    /// Will return standard I/O Exception if failed.
    #[inline]
    pub fn connect(addr: &SocketAddr) -> STDResult<Self> {
        match TcpStream::connect(addr) {
            Ok(r) => {
                let socket = PeerSocket {
                    token: None,
                    stream: r,
                    read_buffer: vec![],
                    write_buffer: vec![],
                    stat: PeerSocketStat::new(),
                    r_prep: false,
                    w_prep: false,
                    retry_times: INIT_RETRY_TIMES,
                    alive: true
                };
                socket.setup_socket();
                Ok(socket)
            },
            Err(e) => Err(e)
        }
    }

    fn setup_socket(&self) {
        self.stream.set_nodelay(true).unwrap();
        self.stream.set_keepalive_ms(Some(KEEP_ALIVE_MS)).unwrap();
    }

    /// Update a mio event loop token
    #[inline]
    pub fn set_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    /// Get the mio event loop token
    #[inline]
    pub fn token(&self) -> Option<Token> {
        self.token.clone()
    }

    /// Try to write socket message to the buffer.
    #[inline]
    pub fn write_msg(&mut self, msg: SocketMessage) -> STDResult<()> {
        // do nothing if the peer was killed
        if !self.is_alive() {
            return Err(Error::new(ErrorKind::Other, "Peer has been killed"));
        }
        // serialize the socket message
        let mut new_data = serde_json::to_string(&msg).unwrap().into_bytes();
        let size = new_data.len();
        // write header
        let header = SocketMessageHeader::new(size);
        header.write_header(&mut self.write_buffer);
        // write body
        if self.write_buffer.len() + new_data.len() > MAX_WRITE_BUFF_SIZE {
            return Err(Error::new(ErrorKind::WouldBlock, "Buffer overflow"));
        }
        self.write_buffer.append(&mut new_data);
        // write buffer is prepared
        self.w_prep = true;

        Ok(())
    }

    /// Send buffer to the socket
    /// Will return `Err(ErrorKind::Interrupted)` if the socket is not ready yet, please try again.
    /// If return another I/O exceptions, socket could be broken.
    #[inline]
    pub fn send_buffer(&mut self) -> STDResult<()> {
        // do nothing if the peer was killed
        if !self.is_alive() {
            return Err(Error::new(ErrorKind::Other, "Peer has been killed"));
        }
        // send to the socket
        match self.stream.write(&self.write_buffer[..]) {
            Ok(size) => {
                // update statistic
                self.stat.notify_send(size);
                // reset retry times
                self.reset_retry_times();
                // clean buffer
                self.write_buffer.drain(0..size);
                // write buffer is not prepared any longer
                if self.write_buffer.len() == 0 {
                    self.w_prep = false;
                }
                Ok(())
            },
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                // try again
                self.retry();
                Err(Error::new(ErrorKind::WouldBlock, ""))
            },
            Err(e) => {
                // kill the peer
                self.kill();
                Err(e)
            }
        }
    }


    /// Try to receive bytes and store them into buffer.
    /// Will return `Err(ErrorKind::WouldBlock)` if the socket is not ready yet, please try again.
    /// If return another I/O exceptions, socket could be broken.
    #[inline]
    pub fn store_buffer(&mut self) -> STDResult<()> {
        // do nothing if the peer was killed
        if !self.is_alive() {
            return Err(Error::new(ErrorKind::Other, "Peer has been killed"));
        }
        let mut temp_buf: [u8; MIO_WINDOW_SIZE] = [0; MIO_WINDOW_SIZE];
        match self.stream.read(&mut temp_buf) {
            Ok(size) => {
                // update statistic
                self.stat.notify_recv(size);
                // reset retry times
                self.reset_retry_times();
                // error out if buffer overflow
                if self.read_buffer.len() + size > MAX_READ_BUFF_SIZE {
                    // kill the peer
                    self.kill();
                    return Err(Error::new(ErrorKind::WouldBlock, "Buffer overflow"))
                }
                // write the buffer
                self.read_buffer.append(&mut temp_buf[..size].to_vec());
                // read buffer is prepared
                self.r_prep = true;

                Ok(())
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // try again
                self.retry();
                Err(Error::new(ErrorKind::WouldBlock, ""))
            },
            Err(e) => {
                // kill the peer
                self.kill();
                Err(e)
            }
        }
    }

    /// Try to read list of socket messages from the buffer.
    #[inline]
    fn read_msg(&mut self) -> STDResult<Vec<SocketMessage>> {
        // do nothing if the peer was killed
        if !self.is_alive() {
            return Err(Error::new(ErrorKind::Other, "Peer has been killed"));
        }

        let buff_size = self.read_buffer.len();
        let mut lines: Vec<Vec<u8>> = vec![];
        loop {
            // try read header
            if let Some(header) = SocketMessageHeader::read_header(&mut self.read_buffer) {
                // if
                if header.body_size() > MSG_PAYLOAD_LEN {
                    // kill the peer
                    self.kill();
                    return Err(Error::new(ErrorKind::InvalidData, "The msg body size is over limit"));
                }
                // check if contains full msg body
                if buff_size < header.body_size() + MSG_HEADER_LEN {
                    break;
                }
                // start parsing message
                // consume header bytes
                &self.read_buffer.drain(..MSG_HEADER_LEN);
                // read msg body
                let line = (&self.read_buffer[..header.body_size()]).to_vec();
                // consume msg bytes
                &self.read_buffer.drain(..header.body_size() as usize);
                lines.push(line);
            } else {
                break;
            }
        }

        // read buffer is not prepared any longer
        self.r_prep = false;

        // deserialize line buffers into socket message
        Ok(
            lines.into_iter().map(|line| {
                let line_string = unsafe { String::from_utf8_unchecked(line) };
                let line_str = line_string.as_str();
                match serde_json::from_str(line_str) {
                    Ok(r) => r,
                    _ => {
                        self.kill();
                        SocketMessage::exception("socket send a bad message, terminate connection")
                    }
                }
            }).collect::<Vec<SocketMessage>>()
        )
    }

    /// Prepared to send data
    #[inline]
    pub fn prepare_to_send_data(&self) -> bool {
        self.w_prep
    }

    /// Prepared to recv message
    #[inline]
    pub fn prepare_to_recv_msg(&self) -> bool {
        self.r_prep
    }

    #[inline]
    fn retry(&mut self) {
        self.retry_times -= 1;
        if self.retry_times <= 0 {
            self.kill();
        }
    }

    #[inline]
    fn reset_retry_times(&mut self) {
        self.retry_times = INIT_RETRY_TIMES;
    }

    #[inline]
    fn kill(&mut self) {
        self.alive = false;
    }

    /// If the peer is alive or not.
    /// The dead peer should do nothing.
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Evented for PeerSocket {
    // Call this function to register the socket to the event loop after initialized.
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> STDResult<()> {
        self.stream.register(poll, token, interest, opts)
    }

    // Call this function to reregister the socket if socket I/O faild.
    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> STDResult<()> {
        self.stream.reregister(poll, token, interest, opts)
    }

    // Call this function before drop the socket, it is optional.
    fn deregister(&self, poll: &Poll) -> STDResult<()> {
        self.stream.deregister(poll)
    }
}

impl Drop for PeerSocket {
    // Shut down connection if socket dropped.
    fn drop(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}