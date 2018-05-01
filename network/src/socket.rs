use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;
use std::io::*;
use std::mem;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;
use bytebuffer::*;

const READ_BUF_LEN: usize = 1024 * 1024;
const WRITE_BUF_LEN: usize = 1024 * 1024;

pub struct PeerSocket {
    stream: TcpStream,
    read_buffer: ByteBuffer,
    write_queue: ByteBuffer,
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
        }
    }

    fn read_to_cache(&mut self) -> Result<()> {
        // the mio reading window is max at 64k (64 * 1024)
        let mut buffer = [0; 64 * 1024];
        let mut is_something_read = false;

        loop {
            match self.stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    self.read_buffer.write(&buffer[0 .. bytes_read]);
                }
                Err(error) => {
                    return if error.kind() == ErrorKind::WouldBlock ||
                        error.kind() == ErrorKind::Interrupted
                        {
                            if is_something_read {
                                Ok(())
                            } else {
                                Ok(())
                            }
                        } else {
                        Err(From::from(error))
                    }
                }
            }
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