use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use bytebuffer::*;
use message::defines::*;

use std::io::*;
use std::mem;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;
use std::rc::Rc;

pub struct PeerSocket {
    stream: TcpStream,
    buffer: ByteBuffer
}

impl PeerSocket {
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        match TcpStream::connect(addr) {
            Ok(r) => Ok(PeerSocket {
                stream: r,
                buffer: ByteBuffer::new()
            }),
            Err(e) => Err(e)
        }
    }

    pub fn send<T>(&mut self, msg: T) where T: MessageCodec {
        unimplemented!()
    }

    pub fn receive(&mut self) -> impl MessageCodec {
        let mut temp_buf: Vec<u8> = vec![];
        match self.stream.read_to_end(&mut temp_buf) {
            Ok(size) => {
                self.buffer.write(&temp_buf[..]);
                SocketMessage::init_ping()
            },
            Err(e) => {
                SocketMessage::init_ping()
            }
        }
    }

    fn fetch_messages_from_buffer(&mut self, size: usize) {
        let mut cur_size = size;
        let mut line: Vec<u8> = vec![];
        loop {
            if cur_size <= 0 {
                break;
            }
            let ch = self.buffer.read_u8();
            if ch == '\n' as u8 {

            } else {
                line.push(ch as u8);
            }
            cur_size -= 1;
        }
    }
}

impl Evented for PeerSocket {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.stream.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.stream.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> Result<()> {
        self.stream.deregister(poll)
    }
}

impl Drop for PeerSocket {
    fn drop(&mut self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}