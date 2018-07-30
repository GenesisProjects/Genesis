use std::ops::Index;
use std::convert::{From, Into};
use std::string::FromUtf8Error;
use std::mem::transmute;

pub enum RLPError {
    RLPErrorUnknown,

    RLPErrorType,

    RLPErrorTagType,

    RLPErrorTagMissing,

    RLPErrorWrongNumParams,

    RLPErrorUTF8,

    RLPEncodingErrorUnencodable,

    RLPDecodingErrorMalformed,
}

#[derive(Clone, Debug)]
pub enum RLP {
    RLPList(Vec<RLP>),
    RLPItem(Vec<u8>),
    RLPEmpty
}

impl Index<usize> for RLP {
    type Output = RLP;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            &RLP::RLPList(ref list) => {
                &list[index]
            },
            _ => panic!("Only [RLPList] can be indexed")
        }
    }
}

impl From<String> for RLP {
    fn from(v: String) -> Self {
        RLP::RLPItem(v.into_bytes())
    }
}

impl Into<Result<String, FromUtf8Error>> for RLP {
    fn into(self) -> Result<String, FromUtf8Error> {
        match self {
            RLP::RLPItem(value) => {
                String::from_utf8(value)
            },
            _ => panic!("Only [RLPItem] can be converted into [String]")
        }
    }
}

impl From<u8> for RLP {
    fn from(v: u8) -> Self {
        RLP::RLPItem(vec![v])
    }
}

impl Into<u8> for RLP {
    fn into(self) -> u8 {
        match self {
            RLP::RLPItem(value) => {
                if value.len() != 1 {
                    panic!("The size of vec should be 1usize")
                } else {
                    value[0]
                }
            },
            _ => panic!("Only [RLPItem] can be converted into [u8]")
        }
    }
}

impl From<u16> for RLP {
    fn from(v: u16) -> Self {
        let bytes: [u8; 2] = unsafe { transmute(v.to_be()) };
        RLP::RLPItem(vec![bytes[0], bytes[1]])
    }
}

impl Into<u16> for RLP {
    fn into(self) -> u16 {
        match self {
            RLP::RLPItem(value) => {
                if value.len() != 2 {
                    panic!("The size of vec should be 2usize")
                } else {
                    (value[1] as u16) + ((value[0] as u16) << 8)
                }
            },
            _ => panic!("Only [RLPItem] can be converted into [u16]")
        }
    }
}

impl From<u32> for RLP {
    fn from(v: u32) -> Self {
        let bytes: [u8; 4] = unsafe { transmute(v.to_be()) };
        RLP::RLPItem(vec![bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl Into<u32> for RLP {
    fn into(self) -> u32 {
        match self {
            RLP::RLPItem(value) => {
                if value.len() != 4 {
                    panic!("The size of vec should be 4usize")
                } else {
                    (value[3] as u32)
                        + ((value[2] as u32) << 8)
                        + ((value[1] as u32) << 16)
                        + ((value[0] as u32) << 24)
                }
            },
            _ => panic!("Only [RLPItem] can be converted into [u32]")
        }
    }
}



pub type EncodedRLP = Vec<u8>;