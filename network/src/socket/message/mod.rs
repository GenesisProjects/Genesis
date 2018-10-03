#![cfg_attr(all(not(test), stage0), feature(float_internals))]

//! This module allows Genesis to send `SocketMessage`.
//! `SocketMessage` will be serialized cto json format by using crate [serde](https://crates.io/crates/serde).
//! Please implement your own **protocal** to build & verify `SocketMessage`.

pub mod defines;
pub mod message_handler;

use byteorder::{BigEndian, ReadBytesExt};
use std::mem::transmute;

/// The socket message header size
pub const MSG_HEADER_LEN: usize = 8;

pub struct SocketMessageHeader {
    body_size: usize
}

impl SocketMessageHeader {
    pub fn new(body_size: usize) -> Self {
        SocketMessageHeader {
            body_size: body_size
        }
    }

    pub fn read_header(buff: &[u8]) -> Option<Self> {
        let buff_size = buff.len();
        if buff_size < MSG_HEADER_LEN {
            None
        } else {
            // read header bytes
            let mut size_buf: [u8; MSG_HEADER_LEN] = [0; MSG_HEADER_LEN];
            size_buf.clone_from_slice(&buff[..MSG_HEADER_LEN]);
            let msg_size = (&size_buf.to_vec()[..]).read_u64::<BigEndian>().unwrap();
            Some(SocketMessageHeader {
                body_size: msg_size as usize
            })
        }
    }

    pub fn write_header(&self, buff: &mut Vec<u8>) {
        let mut header_bytes: [u8; MSG_HEADER_LEN] = unsafe { transmute(self.body_size.to_be()) };
        buff.append(&mut header_bytes.to_vec());
    }

    pub fn body_size(&self) -> usize {
        self.body_size
    }
}