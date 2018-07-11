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

pub const MAX_LINE_CAHCE_LEN: usize = 1024 * 4;

pub enum SocketMessageErr {
    IOFailed,
    LineCacheOverflow
}

pub enum SocketDataErr {
    IOFailed,
}

#[derive(Debug)]
pub struct PeerSocket {
    stream: TcpStream,
    buffer: ByteBuffer,
    line_cache: Vec<u8>
}

impl PeerSocket {
    pub fn new(socket: TcpStream) -> Self {
        PeerSocket {
            stream: socket,
            buffer:  ByteBuffer::new(),
            line_cache: vec![]
        }
    }

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

    pub fn send_data(&mut self, data: &[u8]) -> STDResult<()> {
        self.stream.write_all(data)
    }

    pub fn receive_data(&mut self, remain_size: usize) -> Result<Vec<u8>, SocketDataErr> {
        let mut temp_buf: Vec<u8> = vec![];
        match self.stream.read_to_end(&mut temp_buf) {
            Ok(size) => {
                // if the read size is larger than remain_size, read the overflow bytes
                // into the line cache
                if size > remain_size {
                    self.buffer.write(&temp_buf[.. remain_size]);
                    let vec = temp_buf[remain_size .. size].to_vec();
                    self.line_cache = vec;
                    temp_buf = vec![];
                    match self.buffer.read_to_end(&mut temp_buf) {
                        Ok(r) => Ok(temp_buf),
                        Err(_) => Err(SocketDataErr::IOFailed)
                    }
                } else {
                    self.buffer.write(&temp_buf[..]);
                    temp_buf = vec![];
                    match self.buffer.read_to_end(&mut temp_buf) {
                        Ok(r) => Ok(temp_buf),
                        Err(_) => Err(SocketDataErr::IOFailed)
                    }
                }
            },
            Err(e) => {
                Err(SocketDataErr::IOFailed)
            }
        }
    }

    pub fn send_msg(&mut self, msg: SocketMessage) -> STDResult<()> {
        self.stream.write_all(&msg.encoder()[..])
    }

    pub fn receive_msgs(&mut self) -> Result<Vec<SocketMessage>, SocketMessageErr> {
        let mut temp_buf: Vec<u8> = vec![];
        match self.stream.read_to_end(&mut temp_buf) {
            Ok(size) => {
                self.buffer.write(&temp_buf[..]);
                self.fetch_messages_from_buffer(size)
            },
            Err(e) => {
                Err(SocketMessageErr::IOFailed)
            }
        }
    }

    fn fetch_messages_from_buffer(&mut self, size: usize) -> Result<Vec<SocketMessage>, SocketMessageErr> {
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
                if self.line_cache.len() > MAX_LINE_CAHCE_LEN {
                    self.clean_line_cache();
                    return Err(SocketMessageErr::LineCacheOverflow)
                }
            }
            cur_size -= 1;
        }

        // flush the line cache back into data buffer
        self.flush_line_cache();

        Ok(lines.into_iter().map(|line| {
            let line_str = str::from_utf8(&line);
            match line_str {
                Ok(r) => SocketMessage::decoder(r),
                Err(e) => SocketMessage::exception("cannot parse input string as utf8 encoded")
            }
        }).collect::<Vec<SocketMessage>>())
    }

    fn flush_line_cache(&mut self) {
        self.buffer.write_all(&self.line_cache[..]);
        self.line_cache = vec![];
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