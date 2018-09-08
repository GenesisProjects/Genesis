use chrono::prelude::*;
use common::address::Address as Account;
use common::hash::*;

use rlp::RLPSerialize;
use rlp::types::*;

use gen_core::validator::*;
use gen_core::transaction::Transaction;
use nat::*;
use socket::message::defines::*;
use std::net::SocketAddr;
use super::peer::*;

const MAX_DELAY: i64 = 30i64;

/// Current node status.
#[derive(Debug, Clone)]
pub struct Status {
    /// The sender's public key.
    pub from: Account,
    /// The height to which the message is related.
    pub height: usize,
    /// Hash of the last committed block.
    pub last_hash: Hash,
}

/// Proposal for a new block.
#[derive(Debug, Clone)]
pub struct Propose {
    /// The validator account.
    pub validator: ValidatorId,
    /// The height to which the message is related.
    pub height: usize,
    /// The round to which the message is related.
    pub round: usize,
    /// Hash of the previous block.
    pub prev_hash: Hash,
    /// The list of transactions to include in the next block.
    pub transactions: Vec<Hash>,
}

impl Propose {
    pub fn hash(&self) -> Hash {
        self.encrype_sha256().unwrap().0
    }
}
impl RLPSerialize for Propose {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

/// Pre-vote for a new block.
#[derive(Debug, Clone)]
pub struct Prevote {
    /// The validator account.
    pub validator: ValidatorId,
    /// The height to which the message is related.
    pub height: usize,
    /// The round to which the message is related.
    pub round: usize,
    /// Hash of the corresponding `Propose`.
    pub propose_hash: Hash,
    /// Locked round.
    pub locked_round: usize,
}

/// Pre-commit for a proposal.
#[derive(Debug, Clone)]
pub struct Precommit {
    /// The validator account.
    pub validator: ValidatorId,
    /// The height to which the message is related.
    pub height: usize,
    /// The round to which the message is related.
    pub round: usize,
    /// Hash of the corresponding `Propose`.
    pub propose_hash: Hash,
    /// Hash of the new block.
    pub block_hash: Hash,
    /// Time of the `Precommit`.
    pub time: DateTime<Utc>,
}

/// `RequestData` represents a request for some data to other nodes. Each enum variant will be
/// translated to the corresponding request-message.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequestData {
    /// Represents `ProposeRequest` message.
    Propose(Hash),
    /// Represents `TransactionsRequest` message for `Propose`.
    ProposeTransactions(Hash),
    /// Represents `TransactionsRequest` message for `BlockResponse`.
    BlockTransactions,
    /// Represents `PrevotesRequest` message.
    Prevotes(usize, Hash),
    /// Represents `BlockRequest` message.
    Block(usize),
}

#[derive(Debug)]
pub struct PeerTable {
    pub table: Vec<(Option<Account>, SocketInfo)>,
    pub limit: usize,
}

impl Clone for PeerTable {
    fn clone(&self) -> Self {
        PeerTable {
            table: self.table.iter().map(|peer_info| peer_info.clone()).collect(),
            limit: self.limit,
        }
    }
}

impl PeerTable {
    pub fn new() -> Self {
        // TODO: make limit configuable
        PeerTable {
            table: vec![],
            limit: 512,
        }
    }

    pub fn new_with_hosts(hosts: Vec<String>) -> Self {
        // TODO: make limit configuable
        PeerTable {
            table: hosts
                .into_iter()
                .map(|host| {
                    socket_info(host)
                })
                .filter(|socket_result| {
                    match socket_result {
                        &Ok(_) => true,
                        &Err(_) => false
                    }
                })
                .map(|socket_result| {
                    (None, socket_result.unwrap())
                })
                .collect()
            ,
            limit: 512,
        }
    }

    pub fn table(&self) -> Vec<(Option<Account>, SocketInfo)> {
        self.clone().table
    }
}

/// # ConsensusProtocol
/// **Usage**
/// - basic protocols, implemented to generate [[SocketMessage]]
/// **Member**
/// - 1.    ***vesion***:       current client version.
/// - 2.    ***account***:      current validator account.
/// - 3.    ***key_pair***:     client public/private keypair.
#[derive(Debug, Clone)]
pub struct ConsensusProtocol {
    vesion: String,
}

impl ConsensusProtocol {
    pub fn new() -> Self {
        //TODO: make version number configurable
        ConsensusProtocol {
            vesion: "0.0.0".to_string(),
        }
    }

    fn verify_version(&self, index: usize, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.version_at(index) {
            self.vesion == v
        } else {
            false
        }
    }

    fn verify_account(index: usize, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.account_at(index) {
            v.text.len() == 32
        } else {
            false
        }
    }

    fn verify_timestamp(index: usize, msg: &SocketMessage) -> bool {
        if let Some(v) = msg.timestamp_at(index) {
            (Utc::now() - v).num_seconds() < MAX_DELAY
        } else {
            false
        }
    }

    pub fn verify_propose(&self, msg: &SocketMessage) -> Option<(Propose, Account)> {
        if msg.args().len() < 8 {
            return None;
        }

        if !(self.verify_version(0usize, msg)
            && Self::verify_account(1usize, msg)
            && Self::verify_timestamp(2usize, msg)
            && msg.int_at(3usize).is_some()
            && msg.int_at(4usize).is_some()
            && msg.int_at(5usize).is_some()
            && msg.hash_at(6usize).is_some()) {
            return None;
        }

        let mut transactions = Vec::<Hash>::new();
        for arg in &msg.args()[8..] {
            match arg {
                &SocketMessageArg::Hash { value } => {
                    transactions.push(value);
                }
                _ => { return None; }
            }
        };

        Some((Propose {
            validator: ValidatorId(msg.int_at(3usize).unwrap() as u16),
            height: msg.int_at(4usize).unwrap() as usize,
            round: msg.int_at(5usize).unwrap() as usize,
            prev_hash: msg.hash_at(6usize).unwrap(),
            transactions
        }, msg.account_at(1usize).unwrap()))
    }

    pub fn verify_prevote(&self, msg: &SocketMessage) -> Option<(Prevote, Account)> {
        if msg.args().len() < 8 {
            return None;
        }

        if !(self.verify_version(0usize, msg)
            && Self::verify_account(1usize, msg)
            && Self::verify_timestamp(2usize, msg)
            && msg.int_at(3usize).is_some()
            && msg.int_at(4usize).is_some()
            && msg.int_at(5usize).is_some()
            && msg.hash_at(6usize).is_some()
            && msg.int_at(7usize).is_some()) {
            return None;
        }

        Some((Prevote {
            validator: ValidatorId(msg.int_at(3usize).unwrap() as u16),
            height: msg.int_at(4usize).unwrap() as usize,
            round: msg.int_at(5usize).unwrap() as usize,
            propose_hash: msg.hash_at(6usize).unwrap(),
            locked_round: msg.int_at(7usize).unwrap() as usize,
        }, msg.account_at(1usize).unwrap()))
    }

    pub fn verify_precommit(&self, msg: &SocketMessage) -> Option<(Precommit, Account)> {
        if msg.args().len() < 8 {
            return None;
        }

        if !(self.verify_version(0usize, msg)
            && Self::verify_account(1usize, msg)
            && Self::verify_timestamp(2usize, msg)
            && msg.int_at(3usize).is_some()
            && msg.int_at(4usize).is_some()
            && msg.int_at(5usize).is_some()
            && msg.hash_at(6usize).is_some()
            && msg.hash_at(7usize).is_some()) {
            return None;
        }

        Some((Precommit {
            validator: ValidatorId(msg.int_at(3usize).unwrap() as u16),
            height: msg.int_at(4usize).unwrap() as usize,
            round: msg.int_at(5usize).unwrap() as usize,
            propose_hash: msg.hash_at(6usize).unwrap(),
            block_hash: msg.hash_at(7usize).unwrap(),
            time: msg.timestamp_at(2usize).unwrap(),
        }, msg.account_at(1usize).unwrap()))
    }

    pub fn verify(&self, msg: &SocketMessage) -> bool {
        match msg.event().as_str() {
            "NOTIFY_PROPOSE" => {
                if msg.args().len() < 7 {
                    return false;
                }

                if !(self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)
                    && msg.hash_at(3).is_some()) {
                    return false;
                }

                for arg in &msg.args()[4..] {
                    match arg {
                        &SocketMessageArg::Int { ref value } => {}
                        _ => { return false; }
                    }
                };
                return true;
            }
            "NOTIFY_PREVOTE" => {
                if msg.args().len() < 8 {
                    return false;
                }

                if !(self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)
                    && msg.hash_at(3).is_some()) {
                    return false;
                }

                for arg in &msg.args()[4..] {
                    match arg {
                        &SocketMessageArg::Int { ref value } => {}
                        _ => { return false; }
                    }
                };
                return true;
            }
            "NOTIFY_PRECOMMIT" => {
                if msg.args().len() < 7 {
                    return false;
                }

                if !(self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)
                    && msg.hash_at(3).is_some()
                    && msg.hash_at(4).is_some()) {
                    return false;
                }

                for arg in &msg.args()[5..] {
                    match arg {
                        &SocketMessageArg::Int { ref value } => {}
                        _ => { return false; }
                    }
                };
                return true;
            }
            "NOTIFY_TNX_REQUEST" => {
                return true;
            }
            "NOTIFY_TNX" => {
                return true;
            }
            _ => {
                return true;
            }
        }
    }

    pub fn notify_propose(&self, propose: &Propose) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "NOTIFY_PROPOSE".to_string(),
            vec![],
            vec![],
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << Account::load().expect("Can not load account").into()
            << Utc::now().into()
            << SocketMessageArg::Int {
                value: propose.validator.0 as i64
        } << SocketMessageArg::Int {
            value: propose.height as i64
        } << SocketMessageArg::Int {
            value: propose.round as i64
        } << SocketMessageArg::Hash {
            value: propose.prev_hash
        };

        for tnx in &propose.transactions {
            msg = msg << SocketMessageArg::Hash {
                value: tnx.clone()
            }
        }

        msg
    }

    pub fn notify_prevote(&self, prevote: &Prevote) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "NOTIFY_PROPOSE".to_string(),
            vec![],
            vec![],
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << Account::load().expect("Can not load account").into()
            << Utc::now().into()
            << SocketMessageArg::Int {
            value: prevote.validator.0 as i64
        } << SocketMessageArg::Int {
            value: prevote.height as i64
        } << SocketMessageArg::Int {
            value: prevote.round as i64
        } << SocketMessageArg::Hash {
            value: prevote.propose_hash
        } << SocketMessageArg::Int {
            value: prevote.locked_round as i64
        };

        msg
    }

    pub fn notify_precommit(&self, precommit: &Precommit) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "NOTIFY_PROPOSE".to_string(),
            vec![],
            vec![],
        );

        msg = msg << SocketMessageArg::Vesion {
            value: self.vesion.to_owned()
        } << Account::load().expect("Can not load account").into()
            << Utc::now().into()
            << SocketMessageArg::Int {
            value: precommit.validator.0 as i64
        } << SocketMessageArg::Int {
            value: precommit.height as i64
        } << SocketMessageArg::Int {
            value: precommit.round as i64
        } << SocketMessageArg::Hash {
            value: precommit.propose_hash
        } << SocketMessageArg::Hash {
            value: precommit.block_hash
        } ;

        msg
    }

    pub fn request(&self, req: RequestData) -> SocketMessage {
        let mut msg = SocketMessage::new(
            "REQUEST_".to_string(),
            vec![],
            vec![],
        );

        match req {
            RequestData::Propose(hash) => {
                msg.set_event("REQUEST_PROPOSE".to_string());
                msg = msg << SocketMessageArg::Hash {
                    value: hash
                };

                msg
            }
            RequestData::ProposeTransactions(hash) => {
                msg.set_event("REQUEST_PROPOSE_TNXS".to_string());
                msg = msg << SocketMessageArg::Hash {
                    value: hash
                };

                msg
            }
            RequestData::BlockTransactions => {
                msg.set_event("REQUEST_BLOCK_TNXS".to_string());
                msg
            }
            RequestData::Prevotes(round, hash) => {
                msg.set_event("REQUEST_PREVOTE".to_string());
                msg = msg << SocketMessageArg::Int {
                    value: round as i64
                } << SocketMessageArg::Hash {
                    value: hash
                };

                msg
            }
            _ => {
                msg
            }
        }
    }
}