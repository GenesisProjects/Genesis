use chrono::prelude::*;
use common::address::Address as Account;
use common::hash::Hash;

use gen_core::validator::*;
use gen_core::transaction::Transaction;
use nat::*;
use socket::message::defines::*;
use std::net::SocketAddr;
use super::peer::*;

const MAX_DELAY: i64 = 30i64;

pub trait Notify {
    /// # notify_propose(&mut self, 1)
    /// **Usage**
    /// - send propose message
    /// ## Examples
    /// ```
    /// ```
    fn notify_propose(protocol: ConsensusProtocol, round: usize, propose_hash: Hash, table: &PeerTable);

    /// # notify_prevote(&mut self, 1)
    /// **Usage**
    /// - send prevote message
    /// ## Examples
    /// ```
    /// ```
    fn notify_prevote(protocol: ConsensusProtocol, round: usize, propose_hash: Hash, table: &PeerTable);

    /// # notify_precommit(&mut self, 1)
    /// **Usage**
    /// - send precommit message
    /// ## Examples
    /// ```
    /// ```
    fn notify_precommit(protocol: ConsensusProtocol, round: usize, propose_hash: Hash, block_hash: Hash, table: &PeerTable);

    /// # notify_tnx_request(&mut self, 1)
    /// **Usage**
    /// - send tnxs request message
    /// ## Examples
    /// ```
    /// ```
    fn notify_transactions_request(protocol: ConsensusProtocol, round: usize, propose_hash: Hash, tnxs: Vec<Hash>, table: &PeerTable);

    /// # notify_tnxs(&mut self, 1)
    /// **Usage**
    /// - send raw tnxs message
    /// ## Examples
    /// ```
    /// ```
    fn notify_transactions(protocol: ConsensusProtocol, round: usize, propose_hash: Hash, tnxs: Vec<Hash>, table: &PeerTable);
}

/// Current node status.
#[derive(Debug, Clone)]
pub struct Status {
    /// The sender's public key.
    from: Account,
    /// The height to which the message is related.
    height: i64,
    /// Hash of the last committed block.
    last_hash: Hash,
}

/// Proposal for a new block.
#[derive(Debug, Clone)]
pub struct Propose {
    /// The validator account.
    validator: ValidatorId,
    /// The height to which the message is related.
    height: i64,
    /// The round to which the message is related.
    round: i64,
    /// Hash of the previous block.
    prev_hash: Hash,
    /// The list of transactions to include in the next block.
    transactions: Vec<Hash>,
}

/// Pre-vote for a new block.
#[derive(Debug, Clone)]
pub struct Prevote {
    /// The validator account.
    validator: ValidatorId,
    /// The height to which the message is related.
    height: i64,
    /// The round to which the message is related.
    round: i64,
    /// Hash of the corresponding `Propose`.
    propose_hash: Hash,
    /// Locked round.
    locked_round: i64,
}

/// Pre-commit for a proposal.
#[derive(Debug, Clone)]
pub struct Precommit {
    /// The validator account.
    validator: ValidatorId,
    /// The height to which the message is related.
    height: i64,
    /// The round to which the message is related.
    round: i64,
    /// Hash of the corresponding `Propose`.
    propose_hash: Hash,
    /// Hash of the new block.
    block_hash: Hash,
    /// Time of the `Precommit`.
    time: DateTime<Utc>,
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

    fn verify_propose(&self, index: usize, msg: &SocketMessage) -> Option<Propose> {
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
                &SocketMessageArg::Hash { ref value } => {
                    transactions.push(value);
                }
                _ => { return None; }
            }
        };

        Some(Propose {
            validator: ValidatorId(msg.int_at(3usize).unwrap() as u16),
            height: msg.int_at(4usize).unwrap(),
            round: msg.int_at(5usize).unwrap(),
            prev_hash: msg.hash_at(6usize).unwrap(),
            transactions
        })
    }

    fn verify_prevote(&self, index: usize, msg: &SocketMessage) -> Option<Prevote> {
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

        Some(Prevote {
            validator: ValidatorId(msg.int_at(3usize).unwrap() as u16),
            height: msg.int_at(4usize).unwrap(),
            round: msg.int_at(5usize).unwrap(),
            propose_hash: msg.hash_at(6usize).unwrap(),
            locked_round: msg.int_at(7usize).unwrap(),
        })
    }

    fn verify_precommit(&self, index: usize, msg: &SocketMessage) -> Option<Precommit> {
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

        Some(Precommit {
            validator: ValidatorId(msg.int_at(3usize).unwrap() as u16),
            height: msg.int_at(4usize).unwrap(),
            round: msg.int_at(5usize).unwrap(),
            propose_hash: msg.hash_at(6usize).unwrap(),
            block_hash: msg.hash_at(7usize).unwrap(),
            time: msg.timestamp_at(2usize).unwrap(),
        })
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
}