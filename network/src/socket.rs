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
}

impl PeerSocket {
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        match TcpStream::connect(addr) {
            Ok(r) => Ok(PeerSocket {
                stream: r,
            }),
            Err(e) => Err(e)
        }
    }

    pub fn send<T>(msg: T) where T: MessageCodec {
        unimplemented!()
    }

    pub fn receive() -> impl MessageCodec {
        P2PMessage::PING()
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