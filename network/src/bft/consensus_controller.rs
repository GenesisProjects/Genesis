use chrono::*;
use common::address::Address as Account;
use common::gen_message::*;
use common::hash::*;
use common::observe::Observe;
use common::thread::{Thread, ThreadStatus};
use eventloop::*;
use gen_core::block::Block;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use nat::*;
use socket::message::defines::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::*;
use std::net::*;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use super::protocol::*;

pub struct ConsensusController {
    account: Account,
    height: usize,

    round: usize,
    locked_round: usize,
    locked_propose: Option<Hash>,
    last_hash: Hash,

    proposes: HashMap<Hash, Propose>,
    blocks: HashMap<Hash, Block>,
    prevotes: HashMap<(usize, Hash), Prevote>,
    precommits: HashMap<(usize, Hash), Precommit>,

    // eventloop: NetworkEventLoop,
}

impl ConsensusController {
    ///
    ///
    pub fn launch_controller() {
        ConsensusController::launch::<ConsensusController>("ConsensusController".to_string());
    }

    ///
    ///
    pub fn launch_controller_with_channel(ch: &'static str) {
        ConsensusController::launch::<ConsensusController>(ch.to_string());
    }
}

impl Consensus for ConsensusController {
    fn notify_propose(protocol: BFTProtocol, round: usize, propose_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_prevote(protocol: BFTProtocol, round: usize, propose_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_precommit(protocol: BFTProtocol, round: usize, propose_hash: Hash, block_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_transactions_request(protocol: BFTProtocol, round: usize, propose_hash: Hash, tnxs: Vec<Hash>, table: &PeerTable) {
        unimplemented!()
    }

    fn handle_consensus(&mut self, msg: SocketMessage) {
        unimplemented!()
    }


    fn handle_propose(&mut self, propose: Propose) {
        unimplemented!()
    }

    fn handle_prevote(&mut self, propose: Prevote) {
        unimplemented!()
    }

    fn handle_precommit(&mut self, propose: Precommit) {
        unimplemented!()
    }

    fn handle_transactions_request(&mut self, tnxs: Vec<Hash>) {
        unimplemented!()
    }
}

impl Observe for ConsensusController {
    fn subscribe(&mut self, name: String) {
        unimplemented!()
    }

    fn unsubscribe(&mut self, name: String) {
        unimplemented!()
    }

    fn receive_async(&mut self) -> Option<Message> {
        unimplemented!()
    }

    fn receive_sync(&mut self) -> Message {
        unimplemented!()
    }
}

impl Thread for ConsensusController {
    fn new(name: String) -> Result<Self> {
        unimplemented!()
    }

    fn run(&mut self) -> bool {
        unimplemented!()
    }

    fn set_status(&mut self, status: ThreadStatus) {
        unimplemented!()
    }

    fn msg_handler(&mut self, msg: Message) {
        unimplemented!()
    }

    fn get_status(&self) -> ThreadStatus {
        unimplemented!()
    }
}

impl Drop for ConsensusController {
    fn drop(&mut self) {}
}