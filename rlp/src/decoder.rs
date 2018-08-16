///https://blog.csdn.net/ggq89/article/details/78629008

use defines::*;
use types::*;
use std::io::Result;
use std::mem::*;
use std::io::{Error, ErrorKind};

#[inline]
fn malformed_err() -> (Result<RLP>, usize) { (Err(Error::new(ErrorKind::Other, "Malformed input")), 0) }

/// RLP Decoder
pub struct Decoder;

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
            _ => 0
        }
    }

    fn decode_helper(input: &Vec<u8>, start: usize, end: usize) -> (Result<RLP>, usize) {
        let prefix = input[start];
        let expected_len = end - start + 1;

        match prefix {
            // single byte
            0x00u8 ... 0x7fu8 => {
                if expected_len != 1 {
                    malformed_err()
                } else {
                    (Ok(RLP::RLPItem (vec![prefix])), 1)
                }
            },
            // short string
            0x80u8 ... 0xb7u8 => {
                let l = prefix - SHORT_STRING_PREFIX_BASE;
                let seg_len = 1usize + l as usize;
                if expected_len != seg_len {
                    malformed_err()
                } else {
                    (Ok(RLP::RLPItem (
                        input[start + 1usize .. start + 1usize + l as usize].to_vec()
                    )), seg_len)
                }
            },
            // long string
            0xb8u8 ... 0xbfu8 => {
                let l_total_byte = prefix - LONG_STRING_PREFIX_BASE;
                let mut buffer = [0u8; 8];
                for i in start + 1usize .. start + 1usize + l_total_byte as usize {
                    buffer[i - start - 1usize] = input[i];
                }
                let l = unsafe{ transmute::<[u8; 8], u64>(buffer) as usize };
                let seg_len = 1usize + l_total_byte as usize + l as usize;
                if expected_len != seg_len {
                    malformed_err()
                } else {
                    let offset = start + 1usize + (l_total_byte as usize);
                    (Ok(RLP::RLPItem (
                        input[offset .. offset + l as usize].to_vec()
                    )), seg_len)
                }
            },
            // short list
            0xc0u8 ... 0xf7u8 => {
                let l = prefix - SHORT_LIST_PREFIX_BASE;
                let all_seg_len = 1usize + l as usize;
                if expected_len != all_seg_len {
                    malformed_err()
                } else {
                    let mut cur_pos = 1usize + start;
                    let mut result_list: Vec<RLP> = vec![];

                    loop {
                        if cur_pos > end { break; }

                        let seg_estimated_end = if cur_pos + 8usize > end { end } else { cur_pos + 8usize };
                        let seg_len = Decoder::detect_len(input[cur_pos .. seg_estimated_end + 1].to_vec());
                        let (r_rlp, _) = Decoder::decode_helper(input, cur_pos, cur_pos + seg_len - 1usize);
                        let rlp: Option<RLP> = match r_rlp {
                            Ok(rlp) => Some(rlp),
                            _ => None,
                        };
                        if rlp.is_none() {
                            return malformed_err();
                        } else {
                            result_list.append(&mut vec![rlp.unwrap()]);
                            cur_pos = cur_pos + seg_len;
                        }
                    }
                    (Ok(RLP::RLPList(result_list)), all_seg_len)
                }
            },
            // long list
            0xf8u8 ... 0xffu8 => {
                let l_total_byte = prefix - LONG_LIST_PREFIX_BASE;
                let mut buffer = [0u8; 8];
                for i in 1usize..(1 + l_total_byte) as usize {
                    buffer[i - 1] = input[i];
                }
                let l = unsafe { transmute::<[u8; 8], u64>(buffer) as usize };
                let all_seg_len = 1usize + l_total_byte as usize + l as usize;
                if expected_len != all_seg_len {
                    malformed_err()
                } else {
                    let mut cur_pos = 1usize + start + l_total_byte as usize;
                    let mut result_list: Vec<RLP> = vec![];
                    loop {
                        if cur_pos > end { break; }
                        let seg_estimated_end = if cur_pos + 8usize > end { end } else { cur_pos + 8usize };
                        let seg_len = Decoder::detect_len(input[cur_pos .. seg_estimated_end + 1].to_vec());
                        let (r_rlp, _) = Decoder::decode_helper(input, cur_pos, cur_pos + seg_len - 1usize);
                        let rlp: Option<RLP> = match r_rlp {
                            Ok(rlp) => Some(rlp),
                            Err(_) => None
                        };
                        if rlp.is_none() {
                            return malformed_err();
                        } else {
                            result_list.append(&mut vec![rlp.unwrap()]);
                            cur_pos = cur_pos + seg_len;
                        }
                    }
                    (Ok(RLP::RLPList(result_list)), all_seg_len)
                }
            },
            // default
            _ => malformed_err()
        }
    }

    /// Decode bytes array to RLP, return `None` if failed
    pub fn decode(input: &EncodedRLP) -> Option<RLP> {
        let (r, _) = Decoder::decode_helper(input, 0usize, input.len() - 1usize);
        match r {
            Ok(r) => Some(r),
            Err(_) => None
        }
    }
}

#[cfg(test)]
mod decoder {
    use super::Decoder;
    use super::{RLP, RLPError};

    #[test]
    fn test_item_string() {
        // "dog"
        let rlp = Decoder::decode(&vec![0x83u8, 'd' as u8, 'o' as u8, 'g' as u8]).unwrap();
        let target: RLP = "dog".to_string().into();
        assert_eq!(rlp, target);
    }


    #[test]
    fn test_item_u8() {
        // 0
        let rlp = Decoder::decode(&vec![0x00u8]).unwrap();
        let target: RLP = 0u8.into();
        assert_eq!(rlp, target);

        // 15
        let rlp = Decoder::decode(&vec![0x0fu8]).unwrap();
        let target: RLP = 15u8.into();
        assert_eq!(rlp, target);
    }


    #[test]
    fn test_item_u16() {
        let rlp = Decoder::decode(&vec![0x82u8, 0x04u8, 0x00u8]).unwrap();
        let target: RLP = 1024u16.into();
        assert_eq!(rlp, target);
    }


    #[test]
    fn test_item_u32() {
        let rlp = Decoder::decode(&vec![132u8, 0u8, 0u8, 0u8, 100u8]).unwrap();
        let target: RLP = 100u32.into();
        assert_eq!(rlp, target);
    }


    #[test]
    fn test_list_empty() {
        let rlp = Decoder::decode(&vec![0xc0]).unwrap();
        let target: RLP = RLP::RLPList(vec![]);
        assert_eq!(rlp, target);
    }

    #[test]
    fn test_list_nested() {
        let rlp = Decoder::decode(&vec![0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0]).unwrap();
        let target: RLP = RLP::RLPList(vec![
            RLP::RLPList(vec![]),
            RLP::RLPList(vec![
                RLP::RLPList(vec![])
            ]),
            RLP::RLPList(vec![
                RLP::RLPList(vec![]),
                RLP::RLPList(vec![
                    RLP::RLPList(vec![])
                ]),
            ])
        ]);
        assert_eq!(rlp, target);
    }
}