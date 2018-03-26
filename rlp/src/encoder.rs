///https://blog.csdn.net/ggq89/article/details/78629008

extern crate bytebuffer;

use self::bytebuffer::*;
use defines::*;
use std::io::{Read, Write, Result};
use std::mem::*;

macro_rules! total_bytes {
    ($e:expr) => {
        if ($e << 8) == 0 { 1u8 }
        else if ($e << 16) == 0 { 2u8 }
        else if ($e << 24) == 0 { 3u8 }
        else if ($e << 32) == 0 { 4u8 }
        else if ($e << 40) == 0 { 5u8 }
        else if ($e << 48) == 0 { 6u8 }
        else { 7u8 }
    };
}

struct Encoder {
    buffer: ByteBuffer
}

impl Encoder {
    fn new(size: usize) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.resize(size);
        Encoder { buffer: buffer }
    }

    fn new_with_size() -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.resize(ENCODER_BUFFER_SIZE);
        Encoder { buffer: buffer }
    }
}

impl Encoder {
    fn encode_byte_len(&self, input: u8) -> usize {
        return 1;
    }

    fn encode_byte(&mut self, input: u8) {
        if input > SINGLE_BYTE_MAX_VALUE {
            panic!("Byte value is greater than 0x7f.");
        } else {
            self.buffer.write_u8(input);
        }
    }

    fn encode_short_str_len(& self, input: &str) -> usize {
        return 1 + input.len();
    }

    fn encode_short_str(&mut self, input: &str) {
        if input.len() > SHORT_STRING_MAX_LEN {
            panic!("String length out of range 0-55.");
        } else {
            let prefix: u8 = SHORT_STRING_PREFIX_BASE + input.len() as u8;
            self.buffer.write(&[prefix]);
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
            let len_bytes: [u8; 8] = unsafe { transmute(l.to_be()) };
            for i in 0..l_total_byte {
                self.buffer.write_u8(len_bytes[i as usize]);
            }
            self.buffer.write(input.as_bytes());
        }
    }


}

