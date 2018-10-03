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
use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;
use self::message::defines::*;
use serde_json;
use std::io::{Error, ErrorKind, Read, Write};
use std::io::Result as STDResult;
use std::net::{Shutdown, SocketAddr};

pub mod message;
use self::message::{SocketMessageHeader, MSG_HEADER_LEN};

const MAX_LINE_CAHCE_LEN: usize = 1024 * 1024 * 4;
const MAX_WRITE_BUFF_SIZE: usize = 1024 * 1024 * 1024;

/// The max mio data window size.
pub const MIO_WINDOW_SIZE: usize = 1024;

/// A non-blocking TCP socket between peer and peer.
/// The data send by socket is sealed within [SocketMessage](message/defines/SocketMessage.t.html)..
/// PeerSocket only support `ipv4` address now.
#[derive(Debug)]
pub struct PeerSocket {
    stream: TcpStream,
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
    line_cache: Vec<u8>,
}

impl PeerSocket {
    /// Init a peer socket with mio [TcpStream](../../mio/tcp/TcpStream.t.html)
    #[inline]
    pub fn new(socket: TcpStream) -> Self {
        // set the socket to nodelay mode
        socket.set_nodelay(true).unwrap();

        PeerSocket {
            stream: socket,
            read_buffer: vec![],
            write_buffer: vec![],
            line_cache: vec![],
        }
    }

    /// Establish a connection to the a socket address directly.
    /// Will return standard I/O Exception if failed.
    #[inline]
    pub fn connect(addr: &SocketAddr) -> STDResult<Self> {
        match TcpStream::connect(addr) {
            Ok(r) => Ok(PeerSocket {
                stream: r,
                read_buffer: vec![],
                write_buffer: vec![],
                line_cache: vec![],
            }),
            Err(e) => Err(e)
        }
    }

    /// Send binary data directly into the socket.
    #[deprecated(since="0.1.0", note="please use `send_msg` instead")]
    #[inline]
    pub fn send_data(&mut self, data: &[u8]) -> STDResult<usize> { unimplemented!() }

    /// Receive binary data directly from the socket
    #[deprecated(since="0.1.0", note="please use `receive_msgs` instead")]
    #[inline]
    pub fn receive_data(&mut self, remain_size: usize) -> STDResult<Vec<u8>> {
        unimplemented!()
    }

    /// Send socket message to the socket.
    #[inline]
    pub fn send_msg(&mut self, msg: SocketMessage) -> STDResult<()> {
        // serialize the socket message
        let mut new_data = serde_json::to_string(&msg).unwrap().into_bytes();
        let size = new_data.len();

        // write header
        let header = SocketMessageHeader::new(size);
        header.write_header(&mut self.write_buffer);

        // write body
        self.write_buffer.append(&mut new_data);
        if self.write_buffer.len() > MAX_WRITE_BUFF_SIZE {
            return Err(Error::new(ErrorKind::ConnectionAborted, "Buffer overflow"))
        }

        // send to the socket
        match self.stream.write(&self.write_buffer[..]) {
            Ok(size) => {
                // clean buffer
                self.write_buffer.drain(0..size);
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    /// Receive list of socket messages from the socket.
    /// The remain bytes will buffer in read buffer if not deserialize to socket message.
    /// Will return `Err(ErrorKind::WouldBlock)` if the socket is not ready yet, please try again.
    /// If return another I/O exceptions, socket could be broken.
    #[inline]
    pub fn receive_msgs(&mut self) -> STDResult<Vec<SocketMessage>> {
        let mut temp_buf: [u8; MIO_WINDOW_SIZE] = [0; MIO_WINDOW_SIZE];
        match self.stream.read(&mut temp_buf) {
            Ok(size) => {
                println!("data chunk recieved: {}!!!", size);
                self.read_buffer.append(&mut temp_buf[..size].to_vec());
                self.fetch_messages_from_buffer()
            }
            Err(e) => Err(e)
        }
    }

    // Try to fetch messages from the socket buffer.
    #[inline]
    fn fetch_messages_from_buffer(&mut self) -> STDResult<Vec<SocketMessage>> {
        let buff_size = self.read_buffer.len();
        let mut lines: Vec<Vec<u8>> = vec![];
        loop {
            // try read header
            if let Some(header) = SocketMessageHeader::read_header(&mut self.read_buffer) {
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

        // deserialize line buffers into socket message
        Ok(
            lines.into_iter().map(|line| {
                let line_string = unsafe { String::from_utf8_unchecked(line) };
                let line_str = line_string.as_str();
                match serde_json::from_str(line_str) {
                    Ok(r) => r,
                    _ => SocketMessage::exception("cannot parse msg")
                }
            }).collect::<Vec<SocketMessage>>()
        )
    }

    // Flush the current line buffer back to read_buffer
    #[inline]
    fn flush_line_cache(&mut self) {
        self.read_buffer.write_all(&self.line_cache[..]).unwrap();
        self.line_cache = vec![];
    }

    // Clean the line cache
    #[inline]
    fn clean_line_cache(&mut self) {
        self.line_cache = vec![];
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