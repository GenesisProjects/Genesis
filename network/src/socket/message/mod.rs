#![cfg_attr(all(not(test), stage0), feature(float_internals))]

//! This module allows Genesis to send `SocketMessage`.
//! `SocketMessage` will be serialized cto json format by using crate [serde](https://crates.io/crates/serde).
//! Please implement your own **protocal** to build & verify `SocketMessage`.

pub mod defines;
pub mod message_handler;

use byteorder::{BigEndian, ReadBytesExt};

/// The socket message header size
pub const MSG_HEADER_LEN: usize = 8;

pub struct SocketMessageHeader {
    body_size: usize
}

impl SocketMessageHeader {
    pub fn read_header(buff: &[u8]) -> Option<Self> {
        let buff_size = buffer.len();
        if buff_size < MSG_HEADER_LEN {
            None
        } else {
            // read header bytes
            let mut size_buf: [u8; 8] = [0; 8];
            size_buf.clone_from_slice(buff[..8]);
            let msg_size = (&size_buf.to_vec()[..]).read_u64::<BigEndian>().unwrap();
            Some(SocketMessageHeader {
                body_size: msg_size
            })
        }
    }

    pub fn body_size(&self) -> usize {
        self.body_size
    }
}