use chrono::*;
use nat::*;
use eventloop::*;

use super::peer::*;
use super::protocol::*;
use super::consensus_config::*;

use common::address::Address as Account;
use common::gen_message::*;
use common::thread::{Thread, ThreadStatus};
use common::observe::Observe;
use common::hash::*;

use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::*;
use std::rc::Rc;
use std::sync::{Mutex, Arc, Condvar};
use std::net::*;
use std::str::FromStr;
use std::time::Duration;
use std::thread;

pub type StateRef = Rc<RefCell<NodeState>>;

pub struct NodeState {
    validator_state: Option<ValidatorState>,
    // Round
    height: usize,
    height_start_time: DateTime<Utc>,
    round: usize,
    locked_round: usize,
    locked_propose: Option<Hash>,
    last_hash: Hash,

    // Messages.
    proposes: HashMap<Hash, ProposeState>,
    prevotes: HashMap<(usize, Hash), Votes<Prevote>>,
    precommits: HashMap<(usize, Hash), Votes<Precommit>>,
    //requests: HashMap<RequestData, RequestState>,
    blocks: HashMap<Hash, BlockState>,
    queued_msgs: RefCell<MessageQueue>,
    unknown_txs: HashMap<Hash, Vec<Hash>>,
    unknown_proposes_with_precommits: HashMap<Hash, Vec<(usize, Hash)>>,
}

/// State of a validator-node.
#[derive(Debug, Clone)]
pub struct ValidatorState {
    account: Account,
    owned_prevotes: HashMap<usize, Prevote>,
    owned_precommits: HashMap<usize, Precommit>,
}

/// State of a propose with unknown txs set and block hash
#[derive(Debug)]
pub struct ProposeState {
    propose: Propose,
    unknown_txs: HashSet<Hash>,
    block_hash: Option<Hash>,
    // Whether the message has been saved to the consensus messages' cache or not.
    is_saved: bool,
}

/// State of a block.
#[derive(Clone, Debug)]
pub struct BlockState {
    hash: Hash,
    // Changes that should be made for block committing.
    // patch: Patch,
    txs: Vec<Hash>,
    proposer_id: usize,
}

/// `VoteMessage` trait represents voting messages such as `Precommit` and `Prevote`.
pub trait VoteMessage {
    /// Return validator if of the message.
    fn validator(&self) -> Account;
}

impl VoteMessage for Precommit {
    fn validator(&self) -> Account {
        self.validator()
    }
}

impl VoteMessage for Prevote {
    fn validator(&self) -> Account {
        self.validator()
    }
}

/// Contains voting messages alongside with there validator ids.
#[derive(Debug)]
pub struct Votes<T: VoteMessage> {
    messages: Vec<T>,
    validators: HashSet<Account>,
    count: usize,
}

impl<T> Votes<T>
    where
        T: VoteMessage,
{
    /// Creates a new `Votes` instance with a specified validators number.
    pub fn new(validators_len: usize) -> Self {
        Self {
            messages: Vec::new(),
            validators: HashSet::new(),
            count: 0,
        }
    }

    /// Inserts a new message if it hasn't been inserted yet.
    pub fn insert(&mut self, message: &T) {
        let voter = message.validator();
        if !self.validators.contains(&voter) {
            self.count += 1;
            self.validators.insert(voter.clone());
            self.messages.push(message.clone());
        }
    }

    /// Returns validators.
    pub fn validators(&self) -> &HashSet<Account> {
        &self.validators
    }

    /// Returns number of contained messages.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns messages.
    pub fn messages(&self) -> &Vec<T> {
        &self.messages
    }
}

impl ValidatorState {
    /// Creates new `ValidatorState` with given validator id.
    pub fn new(account: Account) -> Self {
        Self {
            account,
            owned_precommits: HashMap::new(),
            owned_prevotes: HashMap::new(),
        }
    }

    /// Returns validator id.
    pub fn account(&self) -> Account {
        self.account
    }

    /// Sets new validator id.
    pub fn set_validator_account(&mut self, account: Account) {
        self.account = account;
    }

    /// Checks if the node has pre-vote for the specified round.
    pub fn have_prevote(&self, round: usize) -> bool {
        self.our_prevotes.get(&round).is_some()
    }

    /// Clears pre-commits and pre-votes.
    pub fn clear(&mut self) {
        self.owned_precommits.clear();
        self.owned_prevotes.clear();
    }
}

impl ProposeState {
    /// Returns hash of the propose.
    pub fn hash(&self) -> Hash {
        self.propose.hash()
    }

    /// Returns block hash propose was executed.
    pub fn block_hash(&self) -> Option<Hash> {
        self.block_hash
    }

    /// Set block hash on propose execute.
    pub fn set_block_hash(&mut self, block_hash: Hash) {
        self.block_hash = Some(block_hash)
    }

    /// Returns propose-message.
    pub fn message(&self) -> &Propose {
        &self.propose
    }

    /// Returns unknown transactions of the propose.
    pub fn unknown_txs(&self) -> &HashSet<Hash> {
        &self.unknown_txs
    }

    /// Returns `true` if there are unknown transactions in the propose.
    pub fn has_unknown_txs(&self) -> bool {
        !self.unknown_txs.is_empty()
    }

    /// Indicates whether Propose has been saved to the consensus messages cache
    pub fn is_saved(&self) -> bool {
        self.is_saved
    }

    /// Marks Propose as saved to the consensus messages cache
    pub fn set_saved(&mut self, saved: bool) {
        self.is_saved = saved;
    }
}

impl NodeState {
    pub fn new(
        validator: Account,
        last_hash: Hash,
        last_height: usize,
        height_start_time: DateTime<Utc>,
    ) -> Self {
        Self {
            validator_state: validator.map(ValidatorState::new),
            height: last_height,
            height_start_time,
            round: 0,
            locked_round: 0,
            locked_propose: None,
            last_hash,
            proposes: HashMap::new(),
            blocks: HashMap::new(),
            prevotes: HashMap::new(),
            precommits: HashMap::new(),
            unknown_txs: HashMap::new(),
            unknown_proposes_with_precommits: HashMap::new(),
            //requests: HashMap::new(),
        }
    }

    /// Returns `ValidatorState` if the node is validator.
    pub fn validator_state(&self) -> &Option<ValidatorState> {
        &self.validator_state
    }
}