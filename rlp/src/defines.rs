/// Encoder buffer size
pub const ENCODER_BUFFER_SIZE: usize    = 1024 * 1024 * 16;

/// Decoder buffer size
pub const DECODER_BUFFER_SIZE: usize    = 1024 * 1024 * 16;

/// The max prefix value for a single byte encoding
pub const SINGLE_BYTE_MAX_VALUE: u8     = 0x7fu8;

/// The max len for a short string encoding
pub const SHORT_STRING_MAX_LEN: usize   = 55usize;

/// Short string prefix offset
pub const SHORT_STRING_PREFIX_BASE: u8  = 0x80u8;

/// Long string prefix offset
pub const LONG_STRING_PREFIX_BASE: u8   = 0xb7u8;

/// The max len for a short list encoding
pub const SHORT_LIST_MAX_LEN: usize     = 55usize;

/// Short list prefix offset
pub const SHORT_LIST_PREFIX_BASE: u8    = 0xc0u8;

/// Long list prefix offset
pub const LONG_LIST_PREFIX_BASE: u8     = 0xf7u8;