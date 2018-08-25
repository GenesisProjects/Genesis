use chrono::*;
use nat::*;
use eventloop::*;

use super::peer::*;
use super::protocol::*;
use super::consensus_config_config::*;

use common::address::Address as Account;
use common::gen_message::*;
use common::thread::{Thread, ThreadStatus};
use common::observe::Observe;

use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::*;
use std::rc::Rc;
use std::sync::{Mutex, Arc, Condvar};
use std::net::*;
use std::str::FromStr;
use std::time::Duration;
use std::thread;

pub struct NodeState {
    // Round
    height: usize,
    height_start_time: DateTime<Utc>,
    round: usize,
    locked_round: usize,
    locked_propose: Option<Hash>,
    last_hash: Hash,

    // Messages.
    proposes: HashMap<Hash, ProposeState>,
    prevotes: HashMap<(Round, Hash), Votes<Prevote>>,
    precommits: HashMap<(Round, Hash), Votes<Precommit>>,
    requests: HashMap<RequestData, RequestState>,
    blocks: HashMap<Hash, BlockState>,
    queued_msgs: RefCell<MessageQueue>,
    unknown_txs: HashMap<Hash, Vec<Hash>>,
    unknown_proposes_with_precommits: HashMap<Hash, Vec<(Round, Hash)>>,
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
    patch: Patch,
    txs: Vec<Hash>,
    proposer_id: ValidatorId,
}