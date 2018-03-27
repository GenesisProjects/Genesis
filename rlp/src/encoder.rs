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

macro_rules! total_bytes {
    ($e:expr) => {
        if ($e >> 8) == 0 { 1u8 }
        else if ($e >> 16) == 0 { 2u8 }
        else if ($e >> 24) == 0 { 3u8 }
        else if ($e >> 32) == 0 { 4u8 }
        else if ($e >> 40) == 0 { 5u8 }
        else if ($e >> 48) == 0 { 6u8 }
        else { 7u8 }
    };
}

pub struct Encoder {
    buffer: ByteBuffer,
    len_cache: HashMap<String, usize>
}

impl Encoder {
    pub fn new_with_size(size: usize) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.resize(size);
        Encoder { buffer: buffer, len_cache: HashMap::new() }
    }

    pub fn new() -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.resize(ENCODER_BUFFER_SIZE);
        Encoder { buffer: buffer, len_cache: HashMap::new() }
    }
}

impl Encoder {
    fn encode_byte_len(&self, input: u8) -> usize { 1 }

    fn encode_byte(&mut self, input: u8) {
        if input > SINGLE_BYTE_MAX_VALUE {
            panic!("Byte value is greater than 0x7f.");
        } else {
            self.buffer.write_u8(input);
        }
    }

    fn encode_short_str_len(& self, input: &str) -> usize { 1 + input.len() }

    fn encode_short_str(&mut self, input: &str) {
        if input.len() > SHORT_STRING_MAX_LEN {
            panic!("String length out of range 0-55.");
        } else {
            let prefix: u8 = SHORT_STRING_PREFIX_BASE + input.len() as u8;
            self.buffer.write_u8(prefix);
            self.buffer.write(input.as_bytes());
        }
    }

    fn encode_long_str_len(& self, input: &str) -> usize {
        let l = input.len() as u64;
        let l_total_byte = total_bytes!(l);
        return 1usize + l_total_byte as usize + input.len() as usize;
    }

    fn encode_long_str(&mut self, input: &str) {
        if input.len() <= SHORT_STRING_MAX_LEN {
            panic!("String length is no enough for encoding.");
        } else {
            let l = input.len() as u64;
            let l_total_byte = total_bytes!(l);

            let prefix: u8 = LONG_STRING_PREFIX_BASE + l_total_byte;

            self.buffer.write(&[prefix]);
            let len_bytes: [u8; 8] = unsafe { transmute(l.to_le()) };
            for i in 0..l_total_byte {
                self.buffer.write_u8(len_bytes[i as usize]);
            }
            self.buffer.write(input.as_bytes());
        }
    }

    fn encode_item_len(&self, input: &str) -> usize {
        if input.len() == 1 && input.as_bytes()[0usize] <= SINGLE_BYTE_MAX_VALUE {
            self.encode_byte_len( input.as_bytes()[0usize])
        } else if input.len() <= SHORT_STRING_MAX_LEN {
            self.encode_short_str_len(input)
        } else {
            self.encode_long_str_len(input)
        }
    }

    fn encode_item(&mut self, input: &str) {
        if input.len() == 1 && input.as_bytes()[0usize] <= SINGLE_BYTE_MAX_VALUE {
            self.encode_byte( input.as_bytes()[0usize]);
        } else if input.len() <= SHORT_STRING_MAX_LEN {
            self.encode_short_str(input);
        } else {
            self.encode_long_str(input);
        }
    }

    fn encode_list_len(&mut self, path: String, input: &RLP) -> usize {
        let cached_result: Option<usize> = match self.len_cache.entry(path.clone()) {
            Vacant(entry) => None,
            Occupied(entry) => Some(entry.get().clone()),
        };

        match cached_result {
            Some(len) => len,
            None =>  match input {
                &RLP::RLPItem { ref value } => {
                    let ret = self.encode_item_len(value.as_str());
                    self.len_cache.insert(path.clone(),ret);
                    ret
                },
                &RLP::RLPList { ref list } => {
                    let mut total = 0usize;
                    for (i, elem) in list.into_iter().enumerate() {
                        let new_path = path.clone() + format!("{}", i).as_str();
                        total = total + self.encode_list_len(new_path,&elem);
                    }
                    if total <= SHORT_LIST_MAX_LEN {
                        let ret = 1 + total;
                        self.len_cache.insert(path.clone(),ret);
                        ret
                    } else {
                        let ret = 1 + total_bytes!(total as u64) as usize + total as usize;
                        self.len_cache.insert(path.clone(),ret);
                        ret
                    }
                },
            }
        }
    }

    fn encode_list(&mut self, path: String, input: &RLP) {
        match input {
            &RLP::RLPItem { ref value } => {
                self.encode_item(value.as_str());
            },
            &RLP::RLPList { ref list } => {
                let l = self.encode_list_len(path.clone(), input) as u64;
                if l <= SHORT_LIST_MAX_LEN as u64 {
                    let prefix: u8 = LONG_LIST_PREFIX_BASE + l as u8;
                    self.buffer.write_u8(prefix);
                    for (i, elem) in list.into_iter().enumerate() {
                        let new_path = path.clone() + format!("{}", i).as_str();
                        self.encode_list(new_path,elem);
                    }
                } else {
                    let l_total_byte = total_bytes!(l);
                    let prefix: u8 = LONG_STRING_PREFIX_BASE + l_total_byte;
                    self.buffer.write_u8(prefix);

                    let len_bytes: [u8; 8] = unsafe { transmute(l.to_le()) };
                    for i in 0..l_total_byte {
                        self.buffer.write_u8(len_bytes[i as usize]);
                    }

                    for (i, elem) in list.into_iter().enumerate() {
                        let new_path = path.clone() + format!("{}", i).as_str();
                        self.encode_list(new_path,elem);
                    }
                }
            },
        }
    }

    pub fn encode(&mut self, obj: &RLP) -> EncodedRLP {
        self.buffer.clear();
        self.len_cache = HashMap::new();

        let len = self.encode_list_len("".to_string(), obj);
        self.encode_list("".to_string(), obj);
        Vec::from_iter(self.buffer.to_bytes()[0..len].iter().cloned())
    }
}