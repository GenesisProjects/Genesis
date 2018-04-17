/// rfc5389 stun protocol
/// reference: [[https://www.rfc-editor.org/rfc/rfc5389.txt]]

use bytebuffer::*;
use super::defines::stun::*;
use super::SocketInfo;

use std::net::UdpSocket;

macro_rules! msg_class_from_type {
    ($msg_type: expr) => {{
        match ($msg_type).raw_value | STUN_CLASS_MASK {
            0x0000u16 => STUNClassType::Request,
            0x0010u16 => STUNClassType::Indication,
            0x0100u16 => STUNClassType::SuccessResp,
            0x0110u16 => STUNClassType::ErrResponse
        }
    }}
}

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
        STUNManager { in_buffer: in_buffer, out_buffer: out_buffer }
    }
}

impl STUNMessageType {
    fn write_msg_type(&self, manager: &mut STUNManager) {
        manager.out_buffer.write_bits(0, 2);
        manager.out_buffer.write_bits(self.raw_value() as u64, 14);
    }

    fn read_msg_type(manager: &mut STUNManager) -> Self {
        manager.in_buffer.read_bits(2u8);
        match manager.in_buffer.read_bits(14u8) {
            0x0001u64 => STUNMessageType::BindingRequest,
            0x0111u64 => STUNMessageType::BindingResponse,
            0x0111u64 => STUNMessageType::BindingErrorResponse,
            0x0002u64 => STUNMessageType::SharedSecretRequest,
            0x0102u64 => STUNMessageType::SharedSecretResponse,
            0x0112u64 => STUNMessageType::SharedSecretErrorResponse,
            _ => panic!("Unreachable!")
        }
    }
}

impl STUNHeader {
    fn write_header(&self, manager: &mut STUNManager) {

    }

    /*fn read_header(manager: &STUNManager) -> Self {

    }*/
}

pub fn map_external_address_stun(local: &SocketInfo) -> Option<SocketInfo> {
    None
}


