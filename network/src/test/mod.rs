#[cfg(test)]
use mio::net::TcpStream;
#[cfg(test)]
use std::io::Result;

#[cfg(test)]
pub static SERVER_ADDRESS: &'static str = "127.0.0.1:65000";

#[cfg(test)]
pub trait TcpStreamMock {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

#[cfg(test)]
impl TcpStreamMock for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(2)
    }
}

