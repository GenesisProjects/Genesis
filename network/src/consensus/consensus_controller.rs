use chrono::*;
use nat::*;
use eventloop::*;

use super::peer::*;
use super::protocol::*;
use super::node_state::*;
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

pub struct ConsensusController<'a> {
    state: NodeState,
    name: String,
    account: Account,
    listener: TcpListener,
    consensus_public_key: PublicKey,
    consensus_secret_key: SecretKey,
    peer_list: HashMap<Token, PeerRef>,
    config: ConsensusConfig,
    eventloop: NetworkEventLoop<Peer<'a>>,
    last_updated: DateTime<Utc>,
    protocol: ConsensusProtocol,
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
    fn connect(&mut self, addr: SocketInfo, state: &mut NodeState) -> Result<(PeerRef)> {
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                Ok(Rc::new(RefCell::new(Peer::new(stream, &addr, state))))
            },
            Err(e) => Err(e)
        }
    }

    /// Init validators list.
    fn init_peers_table(&mut self) {
        let mut raw_peers_table = self.peer_list.values().map(|peer_ref| {
            peer_ref.borrow().peer_table()
        }).fold(Vec::<(Option<Account>, SocketInfo)>::new(), |mut init, ref mut table: Vec<(Option<Account>,SocketInfo)>| {
            init.append(table);
            init
        });

        // add bootstrap peers
        raw_peers_table.append(&mut self.config.bootstrap_peers());

        // filter out self
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref addr, _)| {
            if let Some(ref account) = *addr {
                account.clone() != self.account
            } else {
                true
            }
        }).collect();

        let sockets = raw_peers_table.into_iter()
            .map(|(ref _account, ref addr)| {
                addr.clone()
            }).collect();

        let peers: Vec<(Token, PeerRef)> = sockets.into_iter()
            .map(|addr| {
                let peer_ref = self.connect(addr, &mut self.state).unwrap();
                thread::sleep(Duration::from_millis(20));
                let ret = (self.eventloop.register_peer(&peer_ref.borrow()), peer_ref.clone());
                ret
            })
            .filter(|result| {
                match &result.0 {
                    &Ok(_) => true,
                    &Err(_) => false
                }
            })
            .map(|result| {
                (result.0.unwrap(), result.1)
            })
            .collect();

        // register new peers to the eventloop, add into peer list
        for (ref token, ref peer_ref) in peers {
            peer_ref.borrow_mut().set_token(token.clone());
            self.peer_list.insert(token.clone(), peer_ref.clone());
        }
    }

    /// Returns current height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns start time of the current height.
    pub fn height_start_time(&self) -> SystemTime {
        self.height_start_time
    }

    /// Returns the current round.
    pub fn round(&self) -> usize {
        self.round
    }

    /// Returns a hash of the last block.
    pub fn last_hash(&self) -> Hash {
        self.last_hash
    }
}

impl Notify for ConsensusController {
    fn notify_propose(protocol: P2PProtocol, round: usize, propose_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_prevote(protocol: P2PProtocol, round: usize, propose_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_precommit(protocol: P2PProtocol, round: usize, propose_hash: Hash, block_hash: Hash, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_transactions_request(protocol: P2PProtocol, round: usize, propose_hash: Hash, tnxs: Vec<Hash>, table: &PeerTable) {
        unimplemented!()
    }

    fn notify_transactions(protocol: P2PProtocol, round: usize, propose_hash: Hash, tnxs: Vec<Hash>, table: &PeerTable) {
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
        let config = ConsensusConfig::load();
        let msg_queue = MessageQueue::new(DEFAULT_MSG_QUEUE_SIZE);

        //TODO: make socket resuseable
        let server = TcpListener::bind(&config.server_addr());
        let account = Account::load();

        match (server, account) {
            (Ok(server), Some(account)) => {
                let mut peer_list = HashMap::<Token, PeerRef>::new();
                let state = NodeState::new(account, None, 0, Utc::now());

                Ok(ConsensusController {
                    state: state,
                    name: name,
                    account: account,
                    listener: server,
                    consensus_public_key: None,
                    consensus_secret_key: None,
                    peer_list: peer_list,
                    config: config,
                    eventloop: NetworkEventLoop::new(config.events_size()),
                    last_updated: Utc::now(),
                    protocol: ConsensusProtocol::new(),
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
        self.init_peers_table();
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