/// Encoder buffer size
pub const ENCODER_BUFFER_SIZE: usize = 1024 * 1024;

/// Decoder buffer size
pub const DECODER_BUFFER_SIZE: usize = 1024 * 1024;

pub const SINGLE_BYTE_MAX_VALUE: u8 = 0x7fu8;

pub const SHORT_STRING_PREFIX_BASE: u8 = 0x80u8;
pub const SHORT_STRING_MAX_LEN: usize = 55usize;

pub const LONG_STRING_PREFIX_BASE: u8 = 0xb7;