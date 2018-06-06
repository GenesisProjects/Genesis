use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;
use common::hash::Hash;
use rlp::RLPSerialize;
use rlp::types::*;

use peer::*;
use frame::*;

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

impl RLPSerialize for P2PMessage {
    fn serialize(&self) -> Result<RLP, RLPError> {
        match self {
            &P2PMessage::Bootstrap(ref addr, ref account) => {
                Ok(RLP::RLPList { list: vec!["Bootstrap".to_string().serialize().unwrap_or(RLP::RLPEmpty),
                                             addr.serialize().unwrap_or(RLP::RLPEmpty),
                                             account.serialize().unwrap_or(RLP::RLPEmpty)] })
            },
            &P2PMessage::Accept(ref addr, ref account, ref blcok_info, ref peer_table) => {
               unimplemented!()
            },
            &P2PMessage::Reject(ref addr, ref account, ref reason) => {
                unimplemented!()
            },
            _ => Err(RLPError::RLPErrorUnknown)
        }
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        match rlp {
            &RLP::RLPList { ref list } => {
                let tag = list[0].to_owned();
                match tag {
                    RLP::RLPItem { value } => {
                        match String::from_utf8(value).unwrap().as_str() {
                            "Bootstrap" => {
                                if list.len() == 2 {
                                    Ok(P2PMessage::Bootstrap(
                                        RLPSerialize::deserialize(&list[1]).ok().unwrap(),

                                        RLPSerialize::deserialize(&list[2]).ok().unwrap()
                                    ))
                                } else {
                                    Err(RLPError::RLPErrorWrongNumParams)
                                }
                            },
                            "Accept" => {
                                unimplemented!()
                            },
                            "Reject" => {
                                unimplemented!()
                            },
                            _ => Err(RLPError::RLPErrorTagType)
                        }
                    },
                    _ => Err(RLPError::RLPErrorTagMissing)
                }
            },
            _ => Err(RLPError::RLPErrorUnknown)
        }
    }
}

impl FrameSerialize for P2PMessage {
    fn serialize(&self) -> Vec<Frame> {
        unimplemented!()
    }
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