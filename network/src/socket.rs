use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;
use std::io::*;
use std::mem;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;
use bytebuffer::*;
use frame::*;

const READ_BUF_LEN: usize = 1024 * 1024;
const WRITE_BUF_LEN: usize = 1024 * 1024;

static mut BUFFER: [u8; 64 * 1024] = [0; 64 * 1024];

pub struct PeerSocket {
    stream: TcpStream,
    read_buffer: ByteBuffer,
    write_queue: ByteBuffer,
    cur_read_buffer_size: usize,
    cur_write_queue_size: usize,
}

impl PeerSocket {
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        let mut read_buf = ByteBuffer::new();
        read_buf.resize(READ_BUF_LEN);
        let mut write_buf = ByteBuffer::new();
        write_buf.resize(WRITE_BUF_LEN);

        match TcpStream::connect(addr) {
            Ok(r) => Ok(PeerSocket {
                stream: r,
                read_buffer: read_buf,
                write_queue: write_buf,
                cur_read_buffer_size: 0usize,
                cur_write_queue_size: 0usize
            }),
            Err(e) => Err(e)
        }
    }

    pub fn wrap_socket_stream(stream: TcpStream) ->Self {
        let mut read_buf = ByteBuffer::new();
        read_buf.resize(READ_BUF_LEN);
        let mut write_buf = ByteBuffer::new();
        write_buf.resize(WRITE_BUF_LEN);

        PeerSocket {
            stream: stream,
            read_buffer: read_buf,
            write_queue: write_buf,
            cur_read_buffer_size: 0usize,
            cur_write_queue_size: 0usize
        }
    }

    pub fn read_stream_to_cache(&mut self) -> Result<usize> {
        // the mio reading window is max at 64k (64 * 1024)
        let mut buffer = [0; 64 * 1024];

        match self.stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let mut index: usize = 0;
                loop {
                    match self.read_buffer.write(&buffer[index .. bytes_read]) {
                        Ok(size) => {
                            if size < bytes_read {
                                self.cur_read_buffer_size += size;
                                index += size;
                            } else {
                                return Ok(bytes_read);
                            }
                        }
                        Err(e) =>  { return Err(From::from(e)); }
                    }
                }
            }
            Err(error) => {
                Err(From::from(error))
            }
        }
    }

    pub fn write_stream_from_cache(&mut self) -> Result<usize> {
        // the genesis writing window is max at 64k (64 * 1024)
        let mut buffer = [0; 64 * 1024];

        match self.write_queue.read(&mut buffer[ .. ]) {
            Ok(bytes_write) => {
                let mut index: usize = 0;
                loop {
                    match self.stream.write(&buffer[index .. bytes_write]) {
                        Ok(size) => {
                            if size < bytes_write {
                                self.cur_write_queue_size -= size;
                                index += size;
                            } else {
                                return Ok(bytes_write);
                            }
                        }
                        Err(e) =>  { return Err(From::from(e)); }
                    }
                }
            }
            Err(error) => {
                Err(From::from(error))
            }
        }

    }

    pub fn read_frames_from_cache(&mut self) -> Vec<Frame> {
        let mut result: Vec<Frame> = vec![];

        let temp = self.read_buffer.read_bytes(self.cur_read_buffer_size);
        let mut reader = SHARED_FRAME_READER.lock().unwrap();
        reader.append_data(&temp);

        loop {
            match reader.read_one_frame() {
                Ok(f) => { result.push(f); },
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => { break; },
                    _ => { continue; }
                }
            }
        }
        result
    }

    pub fn write_frame_to_cache(&mut self, frame: &Frame) -> Result<usize>  {
        unimplemented!()
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