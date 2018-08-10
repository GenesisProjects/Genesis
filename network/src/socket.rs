use bytebuffer::*;
use message::defines::*;
use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;
use serde_json;

use std::io::{Error, ErrorKind, Read, Write};
use std::io::Result as STDResult;
use std::net::{Shutdown, SocketAddr};
use std::mem::transmute;
use std::str;

pub const MAX_LINE_CAHCE_LEN: usize = 1024 * 1024 * 4;
pub const MIO_WINDOW_SIZE: usize = 1024;
pub const MAX_WRITE_BUFF_SIZE: usize = 1024 * 1024 * 1024;

#[derive(Debug)]
pub struct PeerSocket {
    stream: TcpStream,
    read_buffer: ByteBuffer,
    write_buffer: Vec<u8>,
    line_cache: Vec<u8>,
}

impl PeerSocket {
    #[inline]
    pub fn new(socket: TcpStream) -> Self {
        // set the socket to nodelay mode
        socket.set_nodelay(true).unwrap();

        PeerSocket {
            stream: socket,
            read_buffer: ByteBuffer::new(),
            write_buffer: vec![],
            line_cache: vec![],
        }
    }

    #[inline]
    pub fn connect(addr: &SocketAddr) -> STDResult<Self> {
        match TcpStream::connect(addr) {
            Ok(r) => Ok(PeerSocket {
                stream: r,
                read_buffer: ByteBuffer::new(),
                write_buffer: vec![],
                line_cache: vec![],
            }),
            Err(e) => Err(e)
        }
    }

    #[inline]
    pub fn send_data(&mut self, data: &[u8]) -> STDResult<usize> {
        let mut new_data = data[..].to_vec();
        self.write_buffer.append(&mut new_data);
        match self.stream.write(&self.write_buffer[..]) {
            Ok(size) => {
                let residue = &self.write_buffer[size..].to_vec();
                self.write_buffer = residue.to_owned();
                if self.write_buffer.len() > MAX_WRITE_BUFF_SIZE {
                    Err(Error::new(ErrorKind::ConnectionAborted, "Buffer overflow"))
                } else {
                    Ok(size)
                }
            }
            Err(e) => Err(e)
        }
    }

    #[inline]
    pub fn receive_data(&mut self, remain_size: usize) -> STDResult<Vec<u8>> {
        let mut temp_buf: [u8; MIO_WINDOW_SIZE] = [0; MIO_WINDOW_SIZE];
        match self.stream.read(&mut temp_buf) {
            Ok(size) => {
                // if the read size is larger than remain_size, read the overflow bytes
                // into the line cache
                if size > remain_size {
                    self.read_buffer.write(&temp_buf[..remain_size])?;
                    let mut vec = temp_buf[remain_size..size].to_vec();
                    let ret = match self.read_buffer.read_to_end(&mut vec) {
                        Ok(_) => Ok(vec.clone()),
                        Err(e) => Err(e)
                    };
                    self.read_buffer.write(&vec[..])?;
                    ret
                } else {
                    self.read_buffer.write(&temp_buf[..size])?;
                    let mut vec: Vec<u8> = vec![];
                    match self.read_buffer.read_to_end(&mut vec) {
                        Ok(_) => Ok(vec),
                        Err(e) => Err(e)
                    }
                }
            }
            Err(e) => Err(e)
        }
    }

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

    #[inline]
    pub fn receive_msgs(&mut self) -> STDResult<Vec<SocketMessage>> {
        let mut temp_buf: [u8; MIO_WINDOW_SIZE] = [0; MIO_WINDOW_SIZE];
        match self.stream.read(&mut temp_buf) {
            Ok(size) => {
                println!("data chunk recieved: {}!!!", size);
                self.read_buffer.write(&temp_buf[..size]).unwrap();
                self.fetch_messages_from_buffer(size)
            }
            Err(e) => Err(e)
        }
    }

    fn fetch_messages_from_buffer(&mut self, size: usize) -> STDResult<Vec<SocketMessage>> {
        /*let mut cur_size = size;
        let mut lines: Vec<Vec<u8>> = vec![];
        loop {
            if cur_size <= 0 {
                break;
            }
            let ch = self.read_buffer.read_u8();
            if ch == '\n' as u8 {
                println!(
                    "msg recieved: {:?}",
                    String::from_utf8(self.line_cache.clone())
                        .unwrap_or("Malformed".to_string())
                );
                lines.push(self.line_cache.clone());
                self.clean_line_cache();
            } else {
                self.line_cache.push(ch as u8);
                if self.line_cache.len() > MAX_LINE_CAHCE_LEN {
                    self.clean_line_cache();
                    return Err(Error::new(ErrorKind::Other, "Line cache overflow"));
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
                Err(_) => SocketMessage::exception("cannot parse input string as utf8 encoded")
            }
        }).collect::<Vec<SocketMessage>>())*/
        let mut lines: Vec<Vec<u8>> = vec![];
        if size < 8 {

        }

        Ok(lines.into_iter().map(|line| {
            let line_string = unsafe{ String::from_utf8_unchecked(line) };
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