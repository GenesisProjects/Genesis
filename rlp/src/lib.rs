//! Recursive Length Prefix serialization crate.
//!
//! The crate includes following features
//!
//!# 1. Codec
//!
//! ## usage
//! * Encoding/Decoding RLP structure to/from bytes array
//! * Please see detail spec. at: [RLP](https://github.com/ethereum/wiki/wiki/RLP)
//!
//! ## example
//! ```
//! use rlp::encoder::Encoder;
//! use rlp::decoder::Decoder;
//! #[macro_use]
//! use rlp::types::{RLP, RLPError};
//!
//!
//! // Generate a test RLP structure
//! let rlp = RLP::RLPList(vec![
//!     RLP::RLPList(vec![]),
//!     RLP::RLPList(vec![
//!         RLP::RLPList(vec![])
//!     ]),
//!     RLP::RLPList(vec![
//!         RLP::RLPList(vec![]),
//!         RLP::RLPList(vec![
//!             RLP::RLPList(vec![])
//!         ]),
//!    ])
//! ]);
//!
//! // Encode the rlp structure to bytes array
//! let mut encoder = Encoder::new();
//! let result = encoder.encode(&rlp);
//! assert_eq!(result, vec![ 0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0 ]);
//!
//! // Decode the rlp structure from bytes array
//! let decoded_rlp = Decoder::decode(&vec![0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0]).unwrap();
//! assert_eq!(decoded_rlp, rlp);
//! ```
//!
//!# 2. Serilization
//!
//! ## usage
//! * Provide ```RLPSerialize``` trait,
//! structures implemented ```RLPSerialize``` can use `serialize()` to obtain ```RLP``` data structure, which is available to store in bytes array
//! ```RLP``` can use `deserialize()` to obtain structure which implemented ```RLPSerialize```
//!
//! ## example
//! ```
//! use rlp::types::{RLP, RLPError};
//! use rlp::RLPSerialize;
//! struct Student {
//!     name: String,
//!     age: u32,
//!     graduated: bool
//! }
//!
//! impl RLPSerialize for Student {
//!     fn serialize(&self) -> Result<RLP, RLPError> {
//!         Ok(
//!             RLP::RLPList(
//!                 vec!["Student".into(),self.name.to_owned().into(),self.age.into(), self.graduated.into()]
//!             )
//!         )
//!     }
//!
//!     fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
//!         if rlp.len() != 4 {
//!             Err(RLPError::RLPErrorWrongNumParams)
//!         } else {
//!             let m_type: String = rlp[0].clone().into();
//!             if m_type != "Student".to_string() {
//!                 Err(RLPError::RLPErrorType)
//!             } else {
//!                 Ok(
//!                     Student {
//!                         name: rlp[1].clone().into(),
//!                         age: rlp[2].clone().into(),
//!                         graduated: rlp[3].clone().into(),
//!                     }
//!                 )
//!             }
//!         }
//!     }
//! }
//! ```

use self::gen_utils::log_writer::LOGGER;
use std::convert::{From, Into};
use std::string::FromUtf8Error;
use types::{RLP, RLPError};
use types::*;

pub const DOMAIN: &'static str = "rlp";

#[macro_use]
pub extern crate lazy_static;
pub extern crate bytebuffer;
pub extern crate gen_utils;

extern crate serde_json;

pub mod decoder;
pub mod defines;
pub mod encoder;
#[macro_use]
pub mod types;


/// This trait allow implemented struct to be serialized to [RLP](types/enum.RLP.html) form
pub trait RLPSerialize: Sized {
    fn serialize(&self) -> Result<RLP, RLPError>;
    fn deserialize(rlp: &types::RLP) -> Result<Self, RLPError>;
}

impl<T> RLPSerialize for T
    where T: Into<RLP>
    + From<RLP>
    + Clone {
    /// Serialize an object to [RLP](types/enum.RLP.html)
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(self.clone().into())
    }

    /// Deserialize [RLP](types/enum.RLP.html) to an object
    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        Ok(rlp.clone().into())
    }
}

impl RLPSerialize for String {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Ok(self.clone().into())
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        let result: String = rlp.clone().into();
        Ok(result)
    }
}

#[cfg(test)]
mod rlp {
    use super::*;

    #[test]
    fn test_ser_de() {
        #[derive(Debug, Eq, PartialEq)]
        struct Student {
            name: String,
            age: u32,
            graduated: bool,
        }

        impl RLPSerialize for Student {
            fn serialize(&self) -> Result<RLP, RLPError> {
                Ok(
                    rlp_list![
                        "Student".into(),
                        self.name.to_owned().into(),
                        self.age.into(),
                        self.graduated.into()
                    ]
                )
            }

            fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
                if rlp.len() != 4 {
                    Err(RLPError::RLPErrorWrongNumParams)
                } else {
                    let m_type: String = rlp[0].clone().into();
                    if m_type != "Student".to_string() {
                        Err(RLPError::RLPErrorType)
                    } else {
                        Ok(
                            Student {
                                name: rlp[1].clone().into(),
                                age: rlp[2].clone().into(),
                                graduated: rlp[3].clone().into(),
                            }
                        )
                    }
                }
            }
        }

        let test_student: Student = Student {
            name: "Edward".into(),
            age: 27,
            graduated: false
        };
        let rlp = test_student.serialize().unwrap();
        let de_student = Student::deserialize(&rlp).unwrap();
        assert_eq!(de_student, test_student);
    }
}