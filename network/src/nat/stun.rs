/// rfc5389 stun protocol
/// reference: [[https://www.rfc-editor.org/rfc/rfc5389.txt]]

use bytebuffer::*;
use std::net::UdpSocket;

struct STUNManager {
    buffer: ByteBuffer,
}

impl STUNManager {
    pub fn new_with_size(size: usize) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.resize(size);
        STUNManager { buffer: buffer }
    }

    pub fn new() -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.resize(ENCODER_BUFFER_SIZE);
        STUNManager { buffer: buffer }
    }
}



/// STUN datagram
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |0 0|     STUN Message Type     |         Message Length        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Magic Cookie                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                     Transaction ID (96 bits)                  |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///

enum STUNMessageType {

}

struct STUNHeader {

}

impl STUNHeader {
    fn write_header(&self, manager: &mut STUNManager) {

    }
}


