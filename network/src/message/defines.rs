use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;
use common::hash::{Hash, HASH_LEN};
use common::key::Signature;
use rlp::RLPSerialize;
use rlp::types::*;
use rust_base58::{ToBase58, FromBase58};
use peer::*;
/*

#[derive(Debug, Clone)]
pub enum RejectReason {

}

#[derive(Debug, Clone)]
pub enum P2PMessage {
    /// Invoked when bootstrap to peers
    Bootstrap(SocketAddr, Account),
    /// Invoked when peers bootstrap to us if we accept
    Accept(SocketAddr, Account, BlockInfo, PeerTable),
    /// Invoked when peers bootstrap to us if we reject
    Reject(SocketAddr, Account, RejectReason),
}


#[derive(Debug, Clone)]
pub enum ChainMessage {
    /// pre-request for a peer to provide a new chain - start position, current chain length
    ChainSyncInit(Account, Hash, u64),
    /// answer for the pre-request - peer_id, fork point, increased chain length
    ChainSyncInitAnswer(Account, Option<Hash>, u64),
    /// request for download a chain - peer_id, fork point, increased chain length
    ChainSyncRequest(Account, Hash, u64),
    /// resource ready - peer_id, fork point, increased chain length, file size, checksum
    ChainSyncReady(Account, Hash, u64, usize, u32),
    /// blockchain downloaded - peer_id, fork point, increased chain length, file size, checksum
    ChainSyncReceived(Account, Hash, u64, usize, u32),
}

*/

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
                }
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

impl SocketMessage {
    pub fn init_ping() -> Self {
        SocketMessage { event: "PING".to_string(), arg: vec![] }
    }

    pub fn init_exception(reason: &str) -> Self {
        SocketMessage { event: "EXCEPTION".to_string(), arg: vec![SocketMessageArg::String { value: reason.to_string() }] }
    }
}

impl MessageCodec for SocketMessage {
    fn encoder(&self) -> Vec<u8>  {
        vec![]
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
            Self::init_exception("contain unknown args")
        } else {
            SocketMessage {
                event: vec[0].to_string(),
                arg: args
            }
        }
    }
}