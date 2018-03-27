///https://blog.csdn.net/ggq89/article/details/78629008

//TODO: Add cache for recursive lenth calculation.

extern crate bytebuffer;

use self::bytebuffer::*;
use defines::*;
use types::*;
use std::io::{Read, Write, Result};
use std::mem::*;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub struct Decoder {

}

impl Decoder {
    //TODO:
    pub fn decode(input: &Vec<u8>) -> RLP {
        let prefix = input[0];

        match prefix {
            0x00u8 ... 0x7fu8 => {
                RLP::RLPItem { value: String::from_utf8(vec![prefix]).unwrap() }
            },
            0x80u8 ... 0xb7u8 => {
                let l = prefix - 0x80u8;
                RLP::RLPItem { value: String::from_utf8(input[1usize .. (1 + l) as usize].to_vec()).unwrap() }
            },
            0xb8u8 ... 0xbfu8 => {
                let l_total_byte = prefix - 0xb7u8;
                let mut buffer = [0u8; 8];
                for i in 1usize .. (1 + l_total_byte) as usize {
                    buffer[i - 1] = input[i];
                }
                let l = unsafe{ transmute::<[u8; 8], u64>(buffer) as usize };
                let offset = 1usize + (l_total_byte as usize);
                RLP::RLPItem {
                    value: String::from_utf8(
                        input[offset .. offset + l as usize].to_vec()
                    ).unwrap()
                }
            },
            0xc0u8 ... 0xf7u8 => {
                RLP::RLPList { list: vec![] }
            },
            0xf8u8 ... 0xffu8 => {
                RLP::RLPList { list: vec![] }
            },
            _ => {
                RLP::RLPList { list: vec![] }
            }
        }
    }
}