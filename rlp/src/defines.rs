/// Encoder buffer size
pub const ENCODER_BUFFER_SIZE: usize    = 1024 * 1024;

/// Decoder buffer size
pub const DECODER_BUFFER_SIZE: usize    = 1024 * 1024;

pub const SINGLE_BYTE_MAX_VALUE: u8     = 0x7fu8;

pub const SHORT_STRING_MAX_LEN: usize   = 55usize;
pub const SHORT_STRING_PREFIX_BASE: u8  = 0x80u8;
pub const LONG_STRING_PREFIX_BASE: u8   = 0xb7u8;

pub const SHORT_LIST_MAX_LEN: usize     = 55usize;
pub const SHORT_LIST_PREFIX_BASE: u8    = 0xc0u8;
pub const LONG_LIST_PREFIX_BASE: u8     = 0xf7u8;
