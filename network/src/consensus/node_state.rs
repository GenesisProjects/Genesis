use chrono::*;
use nat::*;
use eventloop::*;
use bit_vec::BitVec;

use super::peer::*;
use super::protocol::*;
use super::consensus_config::*;

use common::address::Address as Account;
use common::gen_message::*;
use common::thread::{Thread, ThreadStatus};
use common::observe::Observe;
use common::hash::*;

use gen_core::validator::*;

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
    validators: Vec<Validator>,
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
    requests: HashMap<RequestData, RequestState>,
    blocks: HashMap<Hash, BlockState>,
    queued_msgs: RefCell<MessageQueue>,
    unknown_txs: HashMap<Hash, Vec<Hash>>,
    unknown_proposes_with_precommits: HashMap<Hash, Vec<(usize, Hash)>>,
}

/// State of a vaRefCell<MessageQueue>lidator-node.
#[derive(Debug, Clone)]
pub struct ValidatorState {
    validator_id: ValidatorId,
    owned_prevotes: HashMap<usize, Prevote>,
    owned_precommits: HashMap<usize, Precommit>,
}

/// State of a propose with unknown txs set and block hash
#[derive(Debug)]
pub struct ProposeState {
    propose: Propose,
    unknown_tnxs: HashSet<Hash>,
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

#[derive(Debug)]
struct RequestState {
    // Number of attempts made.
    retries: u16,
    // Nodes that have the required information.
    known_nodes: HashSet<ValidatorId>,
}

/// `VoteMessage` trait represents voting messages such as `Precommit` and `Prevote`.
pub trait VoteMessage: Clone {
    /// Return validator if of the message.
    fn validator(&self) -> ValidatorId;
}

impl VoteMessage for Precommit {
    fn validator(&self) -> ValidatorId {
        self.validator
    }
}

impl VoteMessage for Prevote {
    fn validator(&self) -> ValidatorId {
        self.validator
    }
}

/// Contains voting messages alongside with there validator ids.
#[derive(Debug)]
pub struct Votes<T: VoteMessage> {
    messages: Vec<T>,
    validators: BitVec,
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
            validators: BitVec::from_elem(validators_len, false),
            count: 0
        }
    }

    /// Inserts a new message if it hasn't been inserted yet.
    pub fn insert(&mut self, message: &T) {
        let voter: usize = message.validator().0.into();
        if !self.validators[voter] {
            self.count += 1;
            self.validators.set(voter, true);
            self.messages.push(message.clone());
        }
    }

    /// Returns validators.
    pub fn validators(&self) -> &BitVec {
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
    pub fn new(validator_id: ValidatorId) -> Self {
        Self {
            validator_id,
            owned_precommits: HashMap::new(),
            owned_prevotes: HashMap::new(),
        }
    }

    /// Returns validator id.
    pub fn validator_id(&self) -> ValidatorId {
        self.validator_id
    }

    /// Sets new validator id.
    pub fn set_validator_id(&mut self, validator_id: ValidatorId) {
        self.validator_id = validator_id;
    }

    /// Checks if the node has pre-vote for the specified round.
    pub fn have_prevote(&self, round: usize) -> bool {
        self.owned_prevotes.get(&round).is_some()
    }

    /// Clears pre-commits and pre-votes.
    pub fn clear(&mut self) {
        self.owned_precommits.clear();
        self.owned_prevotes.clear();
    }
}

impl RequestState {
    fn new() -> Self {
        Self {
            retries: 0,
            known_nodes: HashSet::new(),
        }
    }

    fn insert(&mut self, peer: ValidatorId) {
        self.known_nodes.insert(peer);
    }

    fn remove(&mut self, peer: &ValidatorId) {
        self.retries += 1;
        self.known_nodes.remove(peer);
    }

    fn is_empty(&self) -> bool {
        self.known_nodes.is_empty()
    }

    fn peek(&self) -> Option<ValidatorId> {
        self.known_nodes.iter().next().cloned()
    }
}

impl ProposeState {
    /// Returns hash of the propose.
    pub fn hash(&self) -> Hash {
        // Todo gen hash for propose
        // self.propose.hash()
        unimplemented!()
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
    pub fn unknown_tnxs(&self) -> &HashSet<Hash> {
        &self.unknown_tnxs
    }

    /// Returns `true` if there are unknown transactions in the propose.
    pub fn has_unknown_tnxs(&self) -> bool {
        !self.unknown_tnxs.is_empty()
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
        validators: Vec<Validator>,
        validator: Option<ValidatorId>,
        last_hash: Hash,
        last_height: usize,
        height_start_time: DateTime<Utc>,
    ) -> Self {
        Self {
            validators,
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
            queued_msgs: RefCell::new(MessageQueue::new(1024)),
            unknown_txs: HashMap::new(),
            unknown_proposes_with_precommits: HashMap::new(),
            requests: HashMap::new(),
        }
    }

    /// Returns the current validators list.
    pub fn validators(&self) -> &Vec<Validator> {
        &self.validators
    }

    /// Returns `ValidatorState` if the node is validator.
    pub fn validator_state(&self) -> &Option<ValidatorState> {
        &self.validator_state
    }

    /// Returns validator if the node is validator.
    pub fn validator_id(&self) -> Option<ValidatorId> {
        self.validator_state.as_ref().map(|s| s.validator_id())
    }

    /// Checks if the node is a validator.
    pub fn is_validator(&self) -> bool {
        self.validator_state().is_some()
    }

    /// Returns the current height of the node.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the current round of the node.
    pub fn round(&self) -> usize {
        self.round
    }

    /// Returns the locked round of the node.
    pub fn locked_round(&self) -> usize {
        self.locked_round
    }

    /// Returns the current round of the node.
    pub fn last_hash(&self) -> Hash {
        self.last_hash
    }

    /// Returns start time of the current height.
    pub fn height_start_time(&self) -> DateTime<Utc> {
        self.height_start_time
    }

    /// Returns the leader id for the specified round and current height.
    pub fn leader(&self, round: usize) -> ValidatorId {
        ValidatorId(((self.height() + round) % (self.validators().len() as usize)) as u16)
    }

    /// Checks if the node is a leader for the current height and round.
    pub fn is_leader(&self) -> bool {
        self.validator_state()
            .as_ref()
            .map_or(false, |validator| self.leader(self.round()) == validator.validator_id)
    }

    /// Returns public key of a validator identified by id.
    pub fn get_validator_key(&self, id: ValidatorId) -> Option<Account> {
        let id: usize = id.0.into();
        self.validators().get(id).map(|x| x.account_addr())
    }

    /// Adds request data to requests list.
    pub fn add_request(&mut self, id: ValidatorId, req: RequestData) -> Option<RequestData> {
        let mut state = self.requests.entry(req.clone()).or_insert(RequestState::new());
        state.insert(id);
        Some(req)
    }

    /// Adds propose to the proposes list. Returns `ProposeState` if it is a new propose.
    pub fn add_propose(
        &mut self,
        propose: &Propose
    ) -> Result<&mut ProposeState> {
        let unknown_tnxs = HashSet::new();
        Ok(self.proposes.entry(propose.hash()).or_insert_with(||

            // Todo check tnxs pool and insert unknown tnxs
            ProposeState {
                propose: propose.clone(),
                unknown_tnxs,
                block_hash: None,
                is_saved: false,
            }
        ))
    }

    /// Returns propose state with hash.
    pub fn get_propose(&self, hash: &Hash) -> Option<&ProposeState> {
        self.proposes.get(hash)
    }

    /// Adds prevote to the prevotes list. Returns true if it has majority prevotes.
    pub fn add_prevote(&mut self, prevote: &Prevote) -> bool {
        if let Some(ref mut validator_state) = self.validator_state {
            if validator_state.validator_id == prevote.validator {
                if let Some(other) = validator_state
                    .owned_prevotes
                    .insert(prevote.round, prevote.clone())
                    {
                        if other.propose_hash != prevote.propose_hash {
                            panic!("Cannot send a different prevote for the same round");
                        }
                    }
            }
        }

        let key = (prevote.round, prevote.propose_hash);
        let validators_len = self.validators().len();
        let votes = self.prevotes
            .entry(key)
            .or_insert_with(|| Votes::new(validators_len));
        votes.insert(prevote);
        votes.count() >= validators_len * 2 / 3
    }

    /// Returns `true` if this node has prevote for the specified round.
    pub fn have_prevote(&self, propose_round: usize) -> bool {
        if let Some(ref validator_state) = *self.validator_state() {
            validator_state.have_prevote(propose_round)
        } else {
            false
        }
    }

    /// Returns `true` if there are +2/3 pre-votes for the specified round and hash.
    pub fn have_majority_prevotes(&self, round: usize, propose_hash: Hash) -> bool {
        match self.prevotes.get(&(round, propose_hash)) {
            Some(votes) => votes.count() >= self.validators().len() * 2 / 3,
            None => false,
        }
    }

    /// Locks to the propose by calling `lock`. This function is called when node receives
    /// +2/3 pre-votes.
    pub fn handle_majority_prevotes(&mut self, prevote_round: usize, propose_hash: &Hash) {
        // Remove request info
        self.remove_request(&RequestData::Prevotes(prevote_round, *propose_hash));
        // Lock to propose
        if self.locked_round() < prevote_round && self.get_propose(propose_hash).is_some() {
            self.lock(prevote_round, *propose_hash);
        }
    }

    /// Returns ids of validators that that sent pre-votes for the specified propose.
    pub fn known_prevotes(&self, round: usize, propose_hash: &Hash) -> BitVec {
        let len = self.validators().len();
        self.prevotes
            .get(&(round, *propose_hash))
            .map_or_else(|| BitVec::from_elem(len, false), |x| x.validators().clone())
    }

    /// Broadcasts the `Prevote` message to all peers. Returns if has +2/3 `Prevote` for the `Propose`
    pub fn broadcast_prevote(&mut self, round: usize, propose_hash: &Hash) -> bool {
        let validator_id = self.validator_id()
            .expect("called broadcast_prevote in Auditor node.");
        let locked_round = self.locked_round();
        let prevote = Prevote {
            validator: validator_id,
            height: self.height(),
            round,
            propose_hash: *propose_hash,
            locked_round
        };

        let has_majority_prevotes = self.add_prevote(&prevote);

        // Todo cache the `Prevote`, Notify Consensus Controller to broadcast prevote
        unimplemented!();

        has_majority_prevotes
    }

    /// Adds precommit to the precommits list. Returns true if it has majority precommits.
    pub fn add_precommit(&mut self, precommit: &Precommit) -> bool {
        if let Some(ref mut validator_state) = self.validator_state {
            if validator_state.validator_id == precommit.validator {
                if let Some(other) = validator_state
                    .owned_precommits
                    .insert(precommit.round, precommit.clone())
                    {
                        if other.propose_hash != precommit.propose_hash {
                            panic!("Cannot send a different precommit for the same round");
                        }
                    }
            }
        }

        let key = (precommit.round, precommit.block_hash);
        let validators_len = self.validators().len();
        let votes = self.precommits
            .entry(key)
            .or_insert_with(|| Votes::new(validators_len));
        votes.insert(precommit);
        votes.count() >= validators_len * 2 / 3
    }

    /// Returns `true` if there are +2/3 pre-votes for the specified round and hash.
    pub fn have_majority_precommits(&self, round: usize, propose_hash: Hash) -> bool {
        match self.prevotes.get(&(round, propose_hash)) {
            Some(votes) => votes.count() >= self.validators().len() * 2 / 3,
            None => false,
        }
    }

    /// Returns ids of validators that that sent pre-commits for the specified propose.
    pub fn known_precommits(&self, round: usize, propose_hash: &Hash) -> BitVec {
        let len = self.validators().len();
        self.precommits
            .get(&(round, *propose_hash))
            .map_or_else(|| BitVec::from_elem(len, false), |x| x.validators().clone())
    }

    /// Removes the specified request from the pending request list.
    pub fn remove_request(&mut self, data: &RequestData) {
        // TODO: Clear timeout. (ECR-171)
        self.requests.remove(data);
    }

    /// Locks to a specified round.
    pub fn lock(&mut self, prevote_round: usize, propose_hash: Hash) {
        for round in prevote_round..(self.round() + 1) {
            if self.is_validator() && !self.have_prevote(round) {
                self.broadcast_prevote(round, &propose_hash);
            }
        }
    }
}