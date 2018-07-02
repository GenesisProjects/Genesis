use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use bytebuffer::*;
use message::defines::*;

use std::io::Result as STDResult;
use std::io::{Read, Write};
use std::mem;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;
use std::rc::Rc;
use std::str;

pub const MAXT_LINE_CAHCE_LEN: usize = 1024 * 4;

pub enum SocketErr {
    IO_FAILED,
    LINE_CAHCE_OVERFLOW
}

pub struct PeerSocket {
    stream: TcpStream,
    buffer: ByteBuffer,
    line_cache: Vec<u8>
}

impl PeerSocket {
    pub fn connect(addr: &SocketAddr) -> STDResult<Self> {
        match TcpStream::connect(addr) {
            Ok(r) => Ok(PeerSocket {
                stream: r,
                buffer: ByteBuffer::new(),
                line_cache: vec![]
            }),
            Err(e) => Err(e)
        }
    }

    pub fn send<T>(&mut self, msg: SocketMessage) {
        unimplemented!()
    }

    pub fn receive(&mut self) -> Result<Vec<SocketMessage>, SocketErr> {
        let mut temp_buf: Vec<u8> = vec![];
        match self.stream.read_to_end(&mut temp_buf) {
            Ok(size) => {
                self.buffer.write(&temp_buf[..]);
                self.fetch_messages_from_buffer(size)
            },
            Err(e) => {
                Err(SocketErr::IO_FAILED)
            }
        }
    }

    fn fetch_messages_from_buffer(&mut self, size: usize) -> Result<Vec<SocketMessage>, SocketErr> {
        let mut cur_size = size;
        let mut lines: Vec<Vec<u8>> = vec![];
        loop {
            if cur_size <= 0 {
                break;
            }
            let ch = self.buffer.read_u8();
            if ch == '\n' as u8 {
                lines.push(self.line_cache.clone());
                self.clean_line_cache();
            } else {
                self.line_cache.push(ch as u8);
                if self.line_cache.len() > MAXT_LINE_CAHCE_LEN {
                    self.clean_line_cache();
                    return Err(SocketErr::LINE_CAHCE_OVERFLOW)
                }
            }
            cur_size -= 1;
        }
        Ok(lines.into_iter().map(|line| {
            let line_str = str::from_utf8(&line);
            match line_str {
                Ok(r) => SocketMessage::decoder(r),
                Err(e) => SocketMessage::init_exception("cannot parse input string as utf8 encoded")
            }
        }).collect::<Vec<SocketMessage>>())
    }

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
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}