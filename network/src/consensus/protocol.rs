use chrono::prelude::*;
use common::address::Address as Account;
use common::hash::Hash;
use gen_core::transaction::Transaction;
use nat::*;
use socket::message::defines::*;
use std::net::SocketAddr;
use super::peer::*;

const MAX_DELAY: i64 = 30i64;

pub trait Consensus {
    /// # notify_propose(&mut self, 1)
    /// **Usage**
    /// - send propose message
    /// ## Examples
    /// ```
    /// ```
    fn notify_propose(protocol: P2PProtocol, round: usize, propose_hash: Hash, table: &PeerTable);

    /// # notify_prevote(&mut self, 1)
    /// **Usage**
    /// - send prevote message
    /// ## Examples
    /// ```
    /// ```
    fn notify_prevote(protocol: P2PProtocol, round: usize, propose_hash: Hash, table: &PeerTable);

    /// # notify_precommit(&mut self, 1)
    /// **Usage**
    /// - send precommit message
    /// ## Examples
    /// ```
    /// ```
    fn notify_precommit(protocol: P2PProtocol, round: usize, propose_hash: Hash, block_hash: Hash, table: &PeerTable);

    /// # notify_tnx_request(&mut self, 1)
    /// **Usage**
    /// - send tnxs request message
    /// ## Examples
    /// ```
    /// ```
    fn notify_transactions_request(protocol: P2PProtocol, round: usize, propose_hash: Hash, tnxs: Vec<Hash>, table: &PeerTable);

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