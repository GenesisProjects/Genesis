use chrono::prelude::*;
use common::address::Address as Account;
use common::hash::Hash;
use gen_core::transaction::Transaction;
use nat::*;
use socket::message::defines::*;
use std::net::SocketAddr;

pub struct PeerTable;

#[derive(Debug, Clone)]
pub struct BFTProtocol {
    vesion: String,
}

pub trait Consensus {
    /// # notify_propose(&mut self, 1)
    /// **Usage**
    /// - send propose message
    /// ## Examples
    /// ```
    /// ```
    fn notify_propose(protocol: BFTProtocol, round: usize, propose_hash: Hash, table: &PeerTable);

    /// # notify_prevote(&mut self, 1)
    /// **Usage**
    /// - send prevote message
    /// ## Examples
    /// ```
    /// ```
    fn notify_prevote(protocol: BFTProtocol, round: usize, propose_hash: Hash, table: &PeerTable);

    /// # notify_precommit(&mut self, 1)
    /// **Usage**
    /// - send precommit message
    /// ## Examples
    /// ```
    /// ```
    fn notify_precommit(protocol: BFTProtocol, round: usize, propose_hash: Hash, block_hash: Hash, table: &PeerTable);

    /// # notify_tnx_request(&mut self, 1)
    /// **Usage**
    /// - send tnxs request message
    /// ## Examples
    /// ```
    /// ```
    fn notify_transactions_request(protocol: BFTProtocol, round: usize, propose_hash: Hash, tnxs: Vec<Hash>, table: &PeerTable);

    /// # handle_consensus(&mut self, 1)
    /// **Usage**
    /// - handle consensus message
    /// ## Examples
    /// ```
    /// ```
    fn handle_consensus(&mut self, msg: SocketMessage);

    /// # handle_propose(&mut self, 1)
    /// **Usage**
    /// - handle propose message
    /// ## Examples
    /// ```
    /// ```
    fn handle_propose(&mut self, propose: Propose);

    /// # handle_prevote(&mut self, 1)
    /// **Usage**
    /// - handle prevote message
    /// ## Examples
    /// ```
    /// ```
    fn handle_prevote(&mut self, propose: Prevote);

    /// # handle_precommit(&mut self, 1)
    /// **Usage**
    /// - handle precommit message
    /// ## Examples
    /// ```
    /// ```
    fn handle_precommit(&mut self, propose: Precommit);

    /// # handle_tnx_request(&mut self, 1)
    /// **Usage**
    /// - handle tnx request message
    /// ## Examples
    /// ```
    /// ```
    fn handle_transactions_request(&mut self, tnxs: Vec<Hash>);
}


pub struct Propose {
    /// The validator id.
    validator: Hash,
    /// The height to which the message is related.
    height: usize,
    /// The round to which the message is related.
    round: usize,
    /// Hash of the previous block.
    prev_hash: Hash,
    /// The list of transactions to include in the next block.
    transactions: Vec<Hash>,
}

pub struct Prevote {
    /// The validator id.
    validator: Account,
    /// The height to which the message is related.
    height: usize,
    /// The round to which the message is related.
    round: usize,
    /// Hash of the corresponding `Propose`.
    propose_hash: Hash,
    /// Locked round.
    locked_round: usize,
}

pub struct Precommit {
    /// The validator id.
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
    time: DateTime<Utc>
}
