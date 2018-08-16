use std::ops::{Index, Shl};
use std::convert::{From, Into};
use std::string::FromUtf8Error;
use std::mem::transmute;

/// # Usage
///
/// Macro to construct `RLPList`
///
/// # Examples
///
/// ```
/// rlp_list![];
/// rlp_list!["a".into(), "b".into()];
/// rlp_list![rlp_list![1u32.into(), 2u32.into()], "test".into()];
/// ```
#[macro_export]
macro_rules! rlp_list {
    ($( $rlp: expr ),*) => {{
         let mut list: Vec<RLP> = vec![];
         $( list.push($rlp); )*
         RLP::RLPList(list)
    }}
}

/// Encoded bytes array
pub type EncodedRLP = Vec<u8>;

/// RLP error types
#[derive(Debug)]
pub enum RLPError {
    /// Unknown error
    RLPErrorUnknown(&'static str),

    /// Struct type doesn't match the type indicated by RLP
    RLPErrorType,

    /// Data tag doesn't match the tag indicated by RLP
    RLPErrorTagType,

    /// Unable to find the tag field in RLP
    RLPErrorTagMissing,

    /// The length of RLP doesn't match the required one
    RLPErrorWrongNumParams,

    /// Malformed UTF8 coding
    RLPErrorUTF8,

    /// Failed to encode RLP to EncodedRLP
    RLPEncodingErrorUnencodable,

    /// Failed to decode EncodedRLP to RLP
    RLPDecodingErrorMalformed,
}

/// A **data structure** that can be encoded to a bytes array.
/// Genesis block chain use this structure to serialize data.
/// Please refer to: [RLP](https://github.com/ethereum/wiki/wiki/RLP) for more details.
#[derive(Clone, Debug, PartialEq)]
pub enum RLP {
    RLPList(Vec<RLP>),
    RLPItem(Vec<u8>),
    RLPEmpty
}

impl RLP {
    pub fn len(&self) -> usize {
        match self {
            &RLP::RLPList(ref v) => v.len(),
            &RLP::RLPItem(_) => 1,
            _ => 0
        }
    }
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

impl From<bool> for RLP {
    fn from(v: bool) -> Self {
        if v {
            RLP::RLPItem(vec![1u8])
        } else {
            RLP::RLPItem(vec![0u8])
        }
    }
}

impl Into<bool> for RLP {
    fn into(self) -> bool {
        match self {
            RLP::RLPItem(value) => {
                if value.len() != 1 {
                    panic!("The total length of vytes array for bool type must be 1")
                } else {
                    if value[0] == 0u8 {
                        false
                    } else {
                        true
                    }
                }
            },
            _ => panic!("Only [RLPItem] can be converted into [bool]")
        }
    }
}

impl From<String> for RLP {
    fn from(v: String) -> Self {
        RLP::RLPItem(v.into_bytes())
    }
}

impl Into<String> for RLP {
    fn into(self) -> String {
        match self {
            RLP::RLPItem(value) => {
                unsafe { String::from_utf8_unchecked(value) }
            },
            _ => panic!("Only [RLPItem] can be converted into [String]")
        }
    }
}

impl From<&'static str> for RLP {
    fn from(v: &'static str) -> Self {
        RLP::RLPItem(v.to_string().into_bytes())
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

impl From<i32> for RLP {
    fn from(v: i32) -> Self {
        let bytes: [u8; 4] = unsafe { transmute(v.to_be()) };
        RLP::RLPItem(vec![bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl Into<i32> for RLP {
    fn into(self) -> i32 {
        match self {
            RLP::RLPItem(value) => {
                if value.len() != 4 {
                    panic!("The size of vec should be 4usize")
                } else {
                    (value[3] as i32)
                        + ((value[2] as i32) << 8)
                        + ((value[1] as i32) << 16)
                        + ((value[0] as i32) << 24)
                }
            },
            _ => panic!("Only [RLPItem] can be converted into [i32]")
        }
    }
}

impl From<u64> for RLP {
    fn from(v: u64) -> Self {
        let bytes: [u8; 8] = unsafe { transmute(v.to_be()) };
        RLP::RLPItem(vec![
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7]
        ])
    }
}

impl Into<u64> for RLP {
    fn into(self) -> u64 {
        match self {
            RLP::RLPItem(value) => {
                if value.len() != 8 {
                    panic!("The size of vec should be 8usize")
                } else {
                    (value[7] as u64)
                        + ((value[6] as u64) << 8)
                        + ((value[5] as u64) << 16)
                        + ((value[4] as u64) << 24)
                        + ((value[3] as u64) << 32)
                        + ((value[2] as u64) << 40)
                        + ((value[1] as u64) << 48)
                        + ((value[0] as u64) << 56)
                }
            },
            _ => panic!("Only [RLPItem] can be converted into [u64]")
        }
    }
}

impl From<i64> for RLP {
    fn from(v: i64) -> Self {
        let bytes: [u8; 8] = unsafe { transmute(v.to_be()) };
        RLP::RLPItem(vec![
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7]
        ])
    }
}

impl Into<i64> for RLP {
    fn into(self) -> i64 {
        match self {
            RLP::RLPItem(value) => {
                if value.len() != 8 {
                    panic!("The size of vec should be 8usize")
                } else {
                    (value[7] as i64)
                        + ((value[6] as i64) << 8)
                        + ((value[5] as i64) << 16)
                        + ((value[4] as i64) << 24)
                        + ((value[3] as i64) << 32)
                        + ((value[2] as i64) << 40)
                        + ((value[1] as i64) << 48)
                        + ((value[0] as i64) << 56)
                }
            },
            _ => panic!("Only [RLPItem] can be converted into [i64]")
        }
    }
}

impl From<f32> for RLP {
    fn from(v: f32) -> Self {
        unimplemented!()
    }
}

impl Into<f32> for RLP {
    fn into(self) -> f32 {
        unimplemented!()
    }
}

impl From<f64> for RLP {
    fn from(v: f64) -> Self {
        unimplemented!()
    }
}

impl Into<f64> for RLP {
    fn into(self) -> f64 {
        unimplemented!()
    }
}


impl Shl<RLP> for RLP {
    type Output = Self;

    fn shl(self, rlp: RLP) -> RLP {
        if let RLP::RLPList(mut list) = self {
            list.push(rlp);
            RLP::RLPList(list)
        } else {
            panic!("Only [RLPList] can be appended")
        }
    }
}


#[cfg(test)]
mod rlp {
    use super::{RLP, RLPError};

    #[test]
    fn test_item_endian() {
        let rlp: RLP = 0x12345678u32.into();
        let num: u32 = rlp.into();
        assert_eq!(num, 0x12345678u32);
    }
}