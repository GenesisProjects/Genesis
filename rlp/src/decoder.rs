///https://blog.csdn.net/ggq89/article/details/78629008

//TODO: Add cache for recursive lenth calculation.

extern crate bytebuffer;

use self::bytebuffer::*;
use defines::*;
use types::*;
use std::io::{Read, Write, Result};
use std::mem::*;
use std::iter::FromIterator;

pub struct Decoder {}

impl Decoder {
    #[inline]
    fn detect_len(input: Vec<u8>) -> usize {
        let prefix = input[0];
        match input[0] {
            // single byte
            0x00u8...0x7fu8 => { 1 },
            // short string
            0x80u8...0xb7u8 => {
                let l = prefix - 0x80u8;
                l as usize + 1usize
            },
            // long string
            0xb8u8...0xbfu8 => {
                let l_total_byte = prefix - 0xb7u8;
                let mut buffer = [0u8; 8];
                for i in 1usize..(1 + l_total_byte) as usize {
                    buffer[i - 1] = input[i];
                }
                let l = unsafe { transmute::<[u8; 8], u64>(buffer) as usize };
                1usize + l_total_byte as usize + l as usize
            },
            // short list
            0xc0u8...0xf7u8 => {
                let l = prefix - 0xc0u8;
                1usize + l as usize
            },
            // long list
            0xf8u8...0xffu8 => {
                let l_total_byte = prefix - 0xf8u8;
                let mut buffer = [0u8; 8];
                for i in 1usize..(1 + l_total_byte) as usize {
                    buffer[i - 1] = input[i];
                }
                let l = unsafe { transmute::<[u8; 8], u64>(buffer) as usize };
                1usize + l_total_byte as usize + l as usize
            },
            // default
            _ => {
                0
            }
        }
    }

    fn decode_helper(input: &Vec<u8>, start: usize, end: usize) -> (RLP, usize) {
        let prefix = input[start];

        match prefix {
            // single byte
            0x00u8 ... 0x7fu8 => {
                (RLP::RLPItem { value: String::from_utf8(vec![prefix]).unwrap() }, 1)
            },
            // short string
            0x80u8 ... 0xb7u8 => {
                let l = prefix - 0x80u8;
                (RLP::RLPItem { value: String::from_utf8(
                    input[start + 1usize .. start + 11usize + l as usize].to_vec()
                ).unwrap() }, 11usize + l as usize)
            },
            // long string
            0xb8u8 ... 0xbfu8 => {
                let l_total_byte = prefix - 0xb7u8;
                let mut buffer = [0u8; 8];
                for i in start + 1usize .. start + 1usize + l_total_byte as usize {
                    buffer[i - start - 1usize] = input[i];
                }
                let l = unsafe{ transmute::<[u8; 8], u64>(buffer) as usize };
                let offset = start + 1usize + (l_total_byte as usize);
                (RLP::RLPItem {
                    value: String::from_utf8(
                        input[offset .. offset + l as usize].to_vec()
                    ).unwrap()
                }, 1usize + l_total_byte as usize + l as usize)
            },
            // short list
            0xc0u8 ... 0xf7u8 => {
                let l = prefix - 0x80u8;
                let mut cur_pos = 1usize + start;
                let mut result_list: Vec<RLP> = vec![];
                loop {
                    if cur_pos > end { break; }

                    let seg_estimated_end = if cur_pos + 8usize > end { end } else { cur_pos + 8usize };
                    let seg_len = Decoder::detect_len(input[cur_pos .. seg_estimated_end + 1].to_vec().clone());
                    let (rlp, _) = Decoder::decode_helper(input, cur_pos, cur_pos + seg_len - 1usize);
                    result_list.append(&mut vec![rlp]);
                    cur_pos = cur_pos + seg_len;
                }

                (RLP::RLPList { list: result_list.clone() }, 1usize + l as usize)
            },
            // long list
            0xf8u8 ... 0xffu8 => {
                let l_total_byte = prefix - 0xf8u8;
                let mut buffer = [0u8; 8];
                for i in 1usize..(1 + l_total_byte) as usize {
                    buffer[i - 1] = input[i];
                }
                let l = unsafe { transmute::<[u8; 8], u64>(buffer) as usize };
                let mut cur_pos = 1usize + start + l_total_byte as usize;
                let mut result_list: Vec<RLP> = vec![];
                loop {
                    if cur_pos > end { break; }

                    let seg_estimated_end = if cur_pos + 8usize > end { end } else { cur_pos + 8usize };
                    let seg_len = Decoder::detect_len(input[cur_pos .. seg_estimated_end + 1].to_vec().clone());
                    let (rlp, _) = Decoder::decode_helper(input, cur_pos, cur_pos + seg_len - 1usize);
                    result_list.append(&mut vec![rlp]);
                    cur_pos = cur_pos + seg_len;
                }

                (RLP::RLPList { list: result_list.clone() }, 1usize + l_total_byte as usize + l as usize)
            },
            // default
            _ => {
                panic!("Incorrect prefix");
            }
        }
    }

    pub fn decode(input: &EncodedRLP) -> RLP {
        let (r, _) = Decoder::decode_helper(input, 0usize, input.len() - 1usize);
        r
    }
}