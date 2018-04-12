/// rfc5389 stun protocol
/// reference: [[https://www.rfc-editor.org/rfc/rfc5389.txt]]

use bytebuffer::*;
use super::defines::stun::*;

use std::net::UdpSocket;

struct STUNManager {
    in_buffer: ByteBuffer,
    out_buffer: ByteBuffer,
}

impl STUNManager {
    pub fn new() -> Self {
        let mut in_buffer = ByteBuffer::new();
        in_buffer.resize(STUN_IN_BUFFER_SIZE);
        let mut out_buffer = ByteBuffer::new();
        out_buffer.resize(STUN_OUT_BUFFER_SIZE);
        STUNManager { in_buffer: buffer, out_buffer: buffer }
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

    fn read_header(&self, manager: &mut STUNManager) {

    }
}


