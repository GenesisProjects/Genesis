use chrono::prelude::*;
use common::address::Address as Account;
use common::hash::Hash;
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
struct Status {
    /// The sender's public key.
    from: Account,
    /// The height to which the message is related.
    height: usize,
    /// Hash of the last committed block.
    last_hash: Hash,
}

/// Proposal for a new block.
struct Propose {
    /// The validator account.
    validator: Account,
    /// The height to which the message is related.
    height: usize,
    /// The round to which the message is related.
    round: usize,
    /// Hash of the previous block.
    prev_hash: Hash,
    /// The list of transactions to include in the next block.
    transactions: [Hash],
}

/// Pre-vote for a new block.
struct Prevote {
    /// The validator account.
    validator: Account,
    /// The height to which the message is related.
    height: usize,
    /// The round to which the message is related.
    round: usize,
    /// Hash of the corresponding `Propose`.
    propose_hash: Hash,
    /// Locked round.
    locked_round: Round,
}

/// Pre-commit for a proposal.
struct Precommit {
    /// The validator account.
    validator: Account,
    /// The height to which the message is related.
    height: usize,
    /// The round to which the message is related.
    round: usize,
    /// Hash of the corresponding `Propose`.
    propose_hash: Hash,
    /// Hash of the new block.
    block_hash: Hash,
    /// Time of the `Precommit`.
    time: DateTime<Utc>,
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

    pub fn verify(&self, msg: &SocketMessage) -> bool {
        match msg.event().as_str() {
            "NOTIFY_PROPOSE" => {
                if msg.args().len() < 4 {
                    return false;
                }

                if !(self.verify_version(0usize, msg)
                    && Self::verify_account(1usize, msg)
                    && Self::verify_timestamp(2usize, msg)) {
                    return false;
                }

                for arg in &msg.args()[3..] {
                    match arg {
                        &SocketMessageArg::String { ref value } => {}
                        _ => { return false; }
                    }
                };
                return true;
            }
            "NOTIFY_PREVOTE" => {
                return true;
            }
            "NOTIFY_PRECOMMIT" => {
                return true;
            }
            "NOTIFY_TNX_REQUEST" => {
                return true;
            }
            "NOTIFY_TNX" => {
                return true;
            }
        }
    }
}