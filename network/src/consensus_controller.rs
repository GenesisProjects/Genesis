use chrono::*;
use nat::*;
use net_config::*;
use network_eventloop::*;
use peer::*;
use message::protocol::*;
use session::*;

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

    eventloop: NetworkEventLoop,
}

impl ConsensusController {
    /// # launch_controller
    /// **Usage**
    /// - launch the controller with a new thread
    /// - subscribe a interthread channel
    /// ## Examples
    /// ```
    /// ```
    pub fn launch_controller() {
        ConsensusController::launch::<ConsensusController>("ConsensusController".to_string());
    }

    /// # launch_controller_with_channel(1)
    /// **Usage**
    /// - launch the controller with a new thread
    /// - subscribe a interthread channel
    /// **Parameters**
    /// - 1. ***&'static str(ch)***: the interthread channel name
    /// ## Examples
    /// ```
    /// ```
    pub fn launch_controller_with_channel(ch: &'static str) {
        ConsensusController::launch::<ConsensusController>(ch.to_string());
    }

    /// # connect(&mut self, 1)
    /// **Usage**
    /// - connect to a peer with tcp protocol
     /// **Parameters**
    /// - 1. ***SocketInfo(addr)***: the target peer addr
    /// **Return**: [[PeerRef]]
    /// ## Examples
    /// ```
    /// ```
    fn connect(&mut self, addr: SocketInfo) -> Result<(PeerRef)> {
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                Ok(Rc::new(RefCell::new(Peer::new(stream, &addr))))
            },
            Err(e) => Err(e)
        }
    }
}

impl Consensus for ConsensusController {
    fn notify_propose(protocol: P2PProtocol, round: usize, propose_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_prevote(protocol: P2PProtocol, round: usize, propose_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_precommit(protocol: P2PProtocol, round: usize, propose_hash: Hash, block_hash: Hash, table: &PeerTable) {
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
}

impl Observe for ConsensusController {
    fn subscribe(&mut self) {
        let name = self.name.to_owned();
        self.ch_pair = Some(
            MESSAGE_CENTER
                .lock()
                .unwrap()
                .subscribe(name)
                .clone()
        );
    }

    fn unsubscribe(&mut self) {
        let name = self.name.to_owned();
        if let Some(ch_pair) = self.ch_pair.clone() {
            let uid = (*ch_pair).0.lock().unwrap().uid.clone();
            self.ch_pair = None;
            MESSAGE_CENTER
                .lock()
                .unwrap()
                .unsubscribe(name, uid);
        }

    }

    fn receive_async(&mut self) -> Option<Message> {
        if let Some(ch_pair) = self.ch_pair.clone() {
            (*ch_pair).0.lock().unwrap().accept_msg_async()
        } else {
            None
        }
    }

    fn receive_sync(&mut self) -> Message {
        if let Some(ch_pair) = self.ch_pair.clone() {
            let condvar_ref = &((*ch_pair).1);
            let lock_ref = &((*ch_pair).0);
            if let Some(msg) = lock_ref.lock().unwrap().accept_msg_async().clone() {
                msg
            } else {
                loop {
                    let msg = condvar_ref
                        .wait(lock_ref.lock().unwrap())
                        .unwrap()
                        .accept_msg_async();

                    match msg {
                        Some(msg) => { return msg; }
                        None => { continue; }
                    }
                }
            }
        } else {
            panic!("No channel subscribed")
        }
    }
}

impl Thread for ConsensusController {
    fn new(name: String) -> Result<Self> {
        let config = NetConfig::load();

        //TODO: make socket resuseable
        let server = TcpListener::bind(&config.server_addr());
        let account = Account::load();

        match (server, account) {
            (Ok(server), Some(account)) => {
                let mut peer_list = HashMap::<Token, PeerRef>::new();
                Ok(ConsensusController {
                    account: account.clone(),
                    height: 0usize,
                    round: 0usize,
                    locked_round: 0usize,
                    locked_propose: Hash,
                    last_hash: Hash,
                    proposes: HashMap::new(),
                    blocks: HashMap::new(),
                    prevotes: HashMap::new(),
                    precommits: HashMap::new(),
                    eventloop: NetworkEventLoop::new(config.events_size()),
                })
            },
            (Ok(_), None) => {
                Err(Error::from(ErrorKind::ConnectionRefused))
            },
            (Err(e), _) => {
                Err(e)
            }
        }
    }

    fn run(&mut self) -> bool {
        self.eventloop.register_server(&self.listener);
        // fetch the next tick
        let result = self.eventloop.next_tick();
        self.update();
        match self.eventloop.status {
            ThreadStatus::Running => {
                match result {
                    Ok(_size) => {
                        self.process_events();
                        true
                    },
                    Err(e) => {
                        panic!("exception: {:?}", e);
                    }
                }
            },
            ThreadStatus::Stop => false,
            ThreadStatus::Pause => true
        }
    }

    fn set_status(&mut self, status: ThreadStatus) {
        self.eventloop.status = status;
    }
}

impl Drop for ConsensusController {
    fn drop(&mut self) {

    }
}