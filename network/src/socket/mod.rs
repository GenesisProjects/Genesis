//! TCP Socket Wrapper
//!
//! Genesis use this crate to send peer to peer messages.
//! All I/O operations are **asynchronous**
//!
//! # Examples
//! ```
//! use gen_network::socket::PeerSocket;
//! use gen_network::socket::message::defines::*;
//!
//! // Connect to another peer
//! let mut socket = PeerSocket::connect("127.0.0.1:30001".into()).unwrap();
//!
//! // Send a socket message
//! socket.send_msg(SocketMessage::heartbeat());
//!
//! // Recieve socket messages from another peer
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

use byteorder::{BigEndian, ReadBytesExt};
use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;
use self::message::defines::*;
use serde_json;
use std::io::{Error, ErrorKind, Read, Write};
use std::io::Result as STDResult;
use std::mem::transmute;
use std::net::{Shutdown, SocketAddr};
use std::str;

pub mod message;

pub const MAX_LINE_CAHCE_LEN: usize = 1024 * 1024 * 4;
pub const MIO_WINDOW_SIZE: usize = 1024;
pub const MAX_WRITE_BUFF_SIZE: usize = 1024 * 1024 * 1024;

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
        // println!("data1 {:?}", &msg);
        let mut new_data = serde_json::to_string(&msg).unwrap().into_bytes();
        let size = new_data.len() as u64;
        let mut size_bytes: [u8; 8] = unsafe { transmute(size.to_be()) };
        self.write_buffer.append(&mut size_bytes.to_vec());
        self.write_buffer.append(&mut new_data);
        match self.stream.write(&self.write_buffer[..]) {
            Ok(size) => {
                self.write_buffer.drain(0..size);
                if self.write_buffer.len() > MAX_WRITE_BUFF_SIZE {
                    Err(Error::new(ErrorKind::ConnectionAborted, "Buffer overflow"))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e)
        }
    }

    /// Receive list of socket messages from the socket.
    /// Will return `Err(ErrorKind::WouldBlock)` if the socket is not ready yet, should try again.
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

    fn fetch_messages_from_buffer(&mut self) -> STDResult<Vec<SocketMessage>> {
        let buff_size = self.read_buffer.len() as u64;
        let mut lines: Vec<Vec<u8>> = vec![];
        loop {
            // check if contains size bytes
            if buff_size < 8u64 {
                break;
            }
            // read size bytes
            let mut size_buf: [u8; 8] = [0; 8];
            size_buf.clone_from_slice(&self.read_buffer[..8]);
            let msg_size = (&size_buf.to_vec()[..])
                .read_u64::<BigEndian>()
                .unwrap();
            // check if contains full msg body
            if buff_size - 8u64 < msg_size {
                break;
            }

            // start parse
            // consume size bytes
            &self.read_buffer.drain(..8);
            // read msg body
            let line = (&self.read_buffer[..msg_size as usize]).to_vec();
            // consume mesg bytes
            &self.read_buffer.drain(..msg_size as usize);
            lines.push(line);
        }

        Ok(lines.into_iter().map(|line| {
            let line_string = unsafe { String::from_utf8_unchecked(line) };
            let line_str = line_string.as_str();
            match serde_json::from_str(line_str) {
                Ok(r) => r,
                _ => SocketMessage::exception("cannot parse msg")
            }
        }).collect::<Vec<SocketMessage>>())
    }

    #[inline]
    fn flush_line_cache(&mut self) {
        self.read_buffer.write_all(&self.line_cache[..]).unwrap();
        self.line_cache = vec![];
    }

    #[inline]
    fn clean_line_cache(&mut self) {
        self.line_cache = vec![];
    }
}

impl Evented for PeerSocket {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> STDResult<()> {
        self.stream.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> STDResult<()> {
        self.stream.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> STDResult<()> {
        self.stream.deregister(poll)
    }
}

impl Drop for PeerSocket {
    fn drop(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}