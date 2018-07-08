use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::ops::*;

use common::address::Address as Account;
use common::hash::{Hash, HASH_LEN};
use chrono::*;
use rlp::RLPSerialize;
use rlp::types::*;
use rust_base58::{ToBase58, FromBase58};
use peer::*;

pub trait MessageCodec {
    fn encoder(&self) -> Vec<u8>;
    fn decoder(input: &str) -> Self;
}

#[derive(Debug, Clone)]
pub enum SocketMessageArg {
    Int { value: i32 },
    String { value: String },
    Account { value: Account },
    Hash { value: Hash },
    Vesion { value: String },
    Timestamp { value: DateTime<Utc> },
    Unknown
}

impl SocketMessageArg {
    pub fn new(input: &str) -> Self {
        let splits = input.trim().split("@");
        let vec: Vec<&str> = splits.collect();
        if vec.len() != 2 {
            SocketMessageArg::Unknown
        } else {
            match vec[0] {
                "Int" => {
                    match vec[1].to_string().parse::<i32>() {
                        Ok(r) => SocketMessageArg::Int { value: r },
                        _ => SocketMessageArg::Unknown
                    }
                },
                "String" => {
                    SocketMessageArg::String { value: vec[1].to_string() }
                },
                "Account" => {
                    SocketMessageArg::Account { value: Account { text: vec[1].to_string() } }
                },
                "Hash" => {
                    match vec[1].to_string().from_base58() {
                        Ok(r) => {
                            if r.len() == HASH_LEN {
                                let mut temp: [u8; 32] = [0u8; HASH_LEN];
                                let r = &r[..HASH_LEN]; // panics if not enough data
                                temp.copy_from_slice(r);
                                SocketMessageArg::Hash { value: temp }
                            } else {
                                SocketMessageArg::Unknown
                            }
                        },
                        _ => SocketMessageArg::Unknown
                    }
                },
                "Vesion" => {
                    SocketMessageArg::Vesion { value: vec[1].to_string() }
                },
                "Timestamp" => {
                    match Utc.datetime_from_str(
                        vec[1].to_string().as_str(),
                        "%Y-%m-%d-%H-%M-%S-%f"
                    ) {
                        Ok(r) => SocketMessageArg::Timestamp { value: r },
                        _ => SocketMessageArg::Unknown
                    }
                },
                _ => {
                    SocketMessageArg::Unknown
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SocketMessage {
    event: String,
    arg: Vec<SocketMessageArg>
}


impl Shl<SocketMessageArg> for SocketMessage {
    type Output = Self;

    fn shl(self, arg: SocketMessageArg) -> SocketMessage {
        let mut new_self = self.clone();
        new_self.arg.push(arg);
        new_self
    }
}

impl SocketMessage {
    pub fn new(event: String, arg: Vec<SocketMessageArg>) -> Self {
        SocketMessage {
            event: event,
            arg: arg
        }
    }

    pub fn int_at(&self, index: usize) -> Option<i32> {
        match self.arg[index] {
            SocketMessageArg::Int { value } => Some(value),
            _ => None
        }
    }

    pub fn string_at(&self, index: usize) -> Option<String> {
        match &self.arg[index] {
            &SocketMessageArg::String { ref value } => Some(value.clone()),
            _ => None
        }
    }

    pub fn account_at(&self, index: usize) -> Option<Account> {
        match &self.arg[index] {
            &SocketMessageArg::Account { ref value } => Some(value.clone()),
            _ => None
        }
    }

    pub fn hash_at(&self, index: usize) -> Option<Hash> {
        match self.arg[index] {
            SocketMessageArg::Hash { value } => Some(value),
            _ => None
        }
    }

    pub fn version_at(&self, index: usize) -> Option<String> {
        match &self.arg[index] {
            &SocketMessageArg::Vesion { ref value } => Some(value.clone()),
            _ => None
        }
    }

    pub fn timestamp_at(&self, index: usize) -> Option<DateTime<Utc>> {
        match self.arg[index] {
            SocketMessageArg::Timestamp { value } => Some(value),
            _ => None
        }
    }

    pub fn event(&self) -> String {
        self.event.to_owned()
    }

    pub fn args(&self) -> Vec<SocketMessageArg> {
        self.arg.to_owned()
    }

    pub fn heartbeat() -> Self {
        SocketMessage { event: "HEARTBEAT".to_string(), arg: vec![] }
    }

    pub fn exception(reason: &str) -> Self {
        SocketMessage { event: "EXCEPTION".to_string(), arg: vec![SocketMessageArg::String { value: reason.to_string() }] }
    }
}

impl MessageCodec for SocketMessage {
    fn encoder(&self) -> Vec<u8>  {
        let mut result: Vec<u8> = vec![];
        let mut vec = self.event.clone().into_bytes();
        result.append(&mut vec);
        for elem in self.arg.clone() {
            match elem {
                SocketMessageArg::Int { value } => {
                    let mut header = " Int@".to_string().into_bytes();
                    let mut vec = value.to_string().into_bytes();
                    result.append(&mut header);
                    result.append(&mut vec);
                },
                SocketMessageArg::String { ref value } => {
                    let mut header = " String@".to_string().into_bytes();
                    let mut vec = value.clone().into_bytes();
                    result.append(&mut header);
                    result.append(&mut vec);
                },
                SocketMessageArg::Account { ref value } => {
                    let mut header = " Account@".to_string().into_bytes();
                    let mut vec = value.text.clone().into_bytes();
                    result.append(&mut header);
                    result.append(&mut vec);
                },
                SocketMessageArg::Hash { ref value } => {
                    let mut header = " Hash@".to_string().into_bytes();
                    let mut vec = value.to_base58().into_bytes();
                    result.append(&mut header);
                    result.append(&mut vec);
                },
                SocketMessageArg::Vesion { ref value } => {
                    let mut header = " Vesion@".to_string().into_bytes();
                    let mut vec = value.clone().into_bytes();
                    result.append(&mut header);
                    result.append(&mut vec);
                },
                SocketMessageArg::Timestamp { ref value } => {
                    let mut header = " Timestamp@".to_string().into_bytes();
                    let mut vec = value.format("%Y-%m-%d-%H-%M-%S-%f").to_string().into_bytes();
                    result.append(&mut header);
                    result.append(&mut vec);
                },
                _ => ()
            }
        }
        result
    }

    fn decoder(input: &str) -> Self {
        let splits = input.trim().split(" ");
        let vec: Vec<&str> = splits.collect();
        let args: Vec<SocketMessageArg> = vec.clone().into_iter().map(|el| { SocketMessageArg::new(el) }).collect();

        let total_unknown = args.clone().into_iter().fold(0usize, |cur, elem| {
            match elem {
                SocketMessageArg::Unknown => cur + 1,
                _ => cur
            }
        });

        if total_unknown > 0 {
            Self::exception("contain unknown args")
        } else {
            SocketMessage {
                event: vec[0].to_string(),
                arg: args
            }
        }
    }
}