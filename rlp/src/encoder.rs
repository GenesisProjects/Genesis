extern crate bytebuffer;

use self::bytebuffer::ByteBuffer;
use defines::*;

struct Encoder {
    buffer: ByteBuffer
}

impl Encoder {
    fn new() -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.resize(ENCODER_BUFFER_SIZE);
        Encoder { buffer: buffer }
    }
}

impl Encoder {
    fn encode_byte(&self, input: &u8) {
        if *input > SINGLE_BYTE_MAX_VALUE {
            panic!("Byte value is greater than 0x7f.");
        } else {
            self.buffer.write_u8(input);
        }
    }

    fn encode_short_str(&self, input: &str) {
        if input.len() > SHORT_STRING_MAX_LEN {
            panic!("String length out of range 0-55.");
        } else {
            let prefix: u8 = SHORT_STRING_PREFIX_BASE + input.len() as u8;
            self.buffer.write(prefix);
            self.buffer.write(input.as_bytes());
        }
    }

    fn encode_long_str(&self, input: &str) {
        if input.len() > SHORT_STRING_MAX_LEN {
            panic!("String length out of range 0-55.");
        } else {
            let prefix: u8 = SHORT_STRING_PREFIX_BASE + input.len() as u8;
            self.buffer.write(prefix);
            self.buffer.write(input.as_bytes());
        }
    }
}

