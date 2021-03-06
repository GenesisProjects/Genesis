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

/// # P2PController
/// **Usage**
/// - p2p network controller
/// **Member**
/// - 1.    ***account***:              current wallet account.
/// - 2.    ***peer_list***:            all peers that establised session
/// - 3.    ***max_allowed_peers***:    limit of allowed peers
/// - 4.    ***waiting_list***:        peers informed by the goship message
/// - 5.    ***max_waiting_list***:     max num of peers waiting for connection
/// - 6.    ***block_list***:           black list
/// - 7.    ***max_blocked_peers***:    black list size
/// - 8.    ***eventloop***:            instance of [[NetworkEventLoop]]
/// - 9.    ***listener***:             server socket
/// - 10.   ***ch_pair***:              message channel,
/// the only way communicate with other controller/thread
pub struct P2PController {
    name: String,
    account: Account,
    height: usize,
    listener: TcpListener,

    peer_list: HashMap<Token, PeerRef>,
    min_required_peers: usize,
    max_allowed_peers: usize,

    waiting_list: Vec<SocketAddr>,
    max_waiting_list: usize,

    block_list: Vec<SocketAddr>,
    max_blocked_peers: usize,

    ch_pair: Option<Arc<(Mutex<MessageChannel>, Condvar)>>,
    config: NetConfig,
    eventloop: NetworkEventLoop,
    last_updated: DateTime<Utc>,
    protocol: P2PProtocol,
}

impl P2PController {
    /// # launch_controller
    /// **Usage**
    /// - launch the controller with a new thread
    /// - subscribe a interthread channel
    /// ## Examples
    /// ```
    /// ```
    pub fn launch_controller() {
        P2PController::launch::<P2PController>("P2PController".to_string());
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
        P2PController::launch::<P2PController>(ch.to_string());
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

    fn init_peers_table(&mut self) {
        self.peer_list = HashMap::<Token, PeerRef>::new();
    }

    fn search_peers(&self) -> Vec<(Option<Account>, SocketInfo)> {
        let mut raw_peers_table = self.peer_list.values().map(|peer_ref| {
            peer_ref.borrow().peer_table()
        }).fold(Vec::<(Option<Account>, SocketInfo)>::new(), |mut init, ref mut table: Vec<(Option<Account>,SocketInfo)>| {
            init.append(table);
            init
        });

        // add bootstrap peers
        raw_peers_table.append(&mut self.config.bootstrap_peers());

        // filter out identical elements
        raw_peers_table.sort_by(|&(ref addr_a, _), &(ref addr_b, _)| addr_a.partial_cmp(addr_b).unwrap());
        raw_peers_table.dedup_by(|&mut (ref addr_a, _), &mut (ref addr_b, _)| *addr_a == *addr_b);

        // filter out self
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref addr, _)| {
            if let Some(ref account) = *addr {
                account.clone() != self.account
            } else {
                true
            }
        }).collect();

        // filter out in current peer list
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref _account, ref addr)| !self.socket_exist(addr)).collect();

        // filter out in block list
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref _account, ref addr)| !self.socket_blocked(addr)).collect();
        raw_peers_table
    }

    fn socket_exist(&self, addr: &SocketAddr) -> bool {
        match self.peer_list.iter().find(|&(_token, peer_ref)| {
            peer_ref.borrow().addr() == *addr
        }) {
            Some(_) => true,
            _ => false
        }
    }

    fn socket_blocked(&self, addr: &SocketAddr) -> bool {
        match self.block_list.iter().find(|&blocked_addr| {
            *blocked_addr == *addr
        }) {
            Some(_) => true,
            _ => false
        }
    }

    #[inline]
    fn height(&self) -> usize {
        self.height
    }

    fn peers_persist(&self) -> Result<usize> {
        // store to fs?
        unimplemented!()
    }

    fn refresh_waiting_list(&mut self) {
        self.waiting_list = self.search_peers()
            .into_iter()
            .map(|(ref _account, ref addr)| {
                addr.clone()
            }).collect();
    }

    fn fetch_peers_from_waiting_list(&mut self) -> Vec<SocketAddr> {
        let w_len = self.waiting_list.len();
        let size = if w_len + self.peer_list.len() > self.max_allowed_peers {
            self.max_allowed_peers - self.peer_list.len()
        } else {
            w_len
        };
        self.waiting_list.drain(0..size).collect()
    }

    fn get_peer(&self, token: Token) -> Option<PeerRef> {
        self.peer_list.get(&token).map(|inner| {
            inner.clone()
        })
    }

    fn add_peer(&mut self, token: &Token, peer_ref: PeerRef) {
        self.peer_list.insert(token.clone(),peer_ref.clone());
    }

    fn remove_peer(&mut self, token: Token) {
        if let Some(peer_ref) = self.peer_list.remove(&token) {
            self.eventloop.deregister(&peer_ref.borrow());
        }
    }

    fn ban_peer(&mut self, addr: &SocketAddr) {
        while self.block_list.len() > self.max_blocked_peers {
            self.block_list.remove(0);
        }
        self.block_list.push(addr.clone());
    }

    fn has_blocked(&self, addr: &SocketAddr) -> bool {
        let result = self.block_list.iter().find(|blocked_addr| {
            blocked_addr.ip().to_string() == addr.ip().to_string()
        });
        result.is_some()
    }

    fn process_events(&mut self) {
        let mut new_peers: Vec<(Token, PeerRef)> = vec![];

        for event in &(self.eventloop.events) {
            match event.token() {
                SERVER_TOKEN => {
                    println!("server event {:?}", event.token());
                    match self.listener.accept() {
                        Ok((socket, addr)) => {
                            println!("Accepting a new peer");
                            // init peer
                            let mut peer = Peer::new(socket, &addr);
                            if !self.socket_exist(&addr) && !self.has_blocked(&addr) {
                                if let Ok(token) = self.eventloop.register_peer(&mut peer) {
                                    peer.set_token(token.clone());
                                    new_peers.push((token, Rc::new(RefCell::new(peer))));
                                }
                            }
                        },
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            // EAGAIN
                            println!("Socket is not ready anymore, stop accepting");
                        },
                        e => {
                            panic!("{:?}", e)
                        }
                    }
                },
                peer_token => {
                    // process peer event
                    // println!("peer event: token {:?}, {:?}, {}",PEER_TOKEN, event, self.eventloop.round);
                    self.get_peer(peer_token).and_then(|ref mut peer_ref| {
                        peer_ref.borrow_mut().session.set_connect(true);
                        let result = peer_ref.borrow_mut().process(self.name.to_owned());
                        match result {
                            Ok(_) => {},
                            Err(_) => {
                                self.eventloop.reregister_peer(peer_token.clone(), &peer_ref.borrow_mut());
                            }
                        }
                        Some(true)
                    });
                }
            }
        }
        for &(ref token, ref peer) in &new_peers {
            self.add_peer(token, peer.clone());
        }
    }

    /// # update(&mut self, 0)
   /// **Usage**
   /// - maintain peerlist, block untrusted peers
   /// - send heartbeats
   /// - refresh the waiting list if peers are not enough
   /// ## Examples
   /// ```
   /// ```
    fn update(&mut self) {
        let update_timebase = self.config.update_timebase();
        if (Utc::now() - self.last_updated).num_milliseconds() < update_timebase {
            return;
        }
        self.last_updated = Utc::now();

        // find aborted token in the peer list
        let aborted_tokens: Vec<Token> = self.peer_list.iter().filter(|pair| {
            match pair.1.borrow().status() {
                SessionStatus::Abort => true,
                _ => false
            }
        }).map(|pair| {
            pair.0.clone()
        }).collect();

        // remove all aborted tokens from the peer list
        for token in aborted_tokens {
            let result = self.get_peer(token.clone()).unwrap();
            let _addr = result.borrow().addr();
            self.remove_peer(token);
        }

        let expire = self.config.peer_expire();
        // find all expired token in the peer list
        let expired_tokens: Vec<Token> = self.peer_list.iter().filter(|pair| {
            if pair.1.borrow().session.milliseconds_from_last_update() > expire {
                true
            } else {
                false
            }
        }).map(|pair| {
            pair.0.clone()
        }).collect();

        // remove all expired tokens from the peer list
        for token in expired_tokens {
            let result = self.get_peer(token.clone()).unwrap();
            let _addr = result.borrow().addr();
            self.remove_peer(token);
        }

        // find all connection timeout tokens in the peer list
        let timeout = self.config.connect_timeout();
        let timeout_tokens: Vec<Token> = self.peer_list.iter().filter(|pair| {
            if pair.1.borrow().session.milliseconds_connecting() > timeout {
                true
            } else {
                false
            }
        }).map(|pair| {
            pair.0.clone()
        }).collect();

        // remove all connection timeout tokens from the peer list
        for token in timeout_tokens {
            let result = self.get_peer(token.clone()).unwrap();
            let _addr = result.borrow().addr();
            self.remove_peer(token);
        }

        // find untrusted tokens in the peer list
        let untrusted_tokens: Vec<Token> = self.peer_list.iter().filter(|pair| {
            pair.1.borrow().credit() == 0
        }).map(|pair| {
            pair.0.clone()
        }).collect();

        // remove all untrusted tokens from the peer list
        for token in untrusted_tokens {
            let result = self.get_peer(token.clone()).unwrap();
            let addr = result.borrow().addr();
            self.remove_peer(token);
            self.block_list.push(addr);
            if self.block_list.len() > self.max_blocked_peers {
                self.block_list.remove(0);
            }
        }

        // if the peer table is too small then refresh it.
        if self.peer_list.len() < self.min_required_peers {
            if self.waiting_list.len() < self.min_required_peers {
                self.refresh_waiting_list();
            }
            let sockets = self.fetch_peers_from_waiting_list();
            let peers: Vec<(Token, PeerRef)> = sockets.into_iter()
                .map(|addr| {
                    let peer_ref = self.connect(addr).unwrap();
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

            // bootstrap all peers at init status
            let hosts: Vec<String> = self.peer_list.iter()
                .map(|pair| {
                    pair.1.borrow().addr().to_string()
                }).collect();
            let table = PeerTable::new_with_hosts(hosts);

            for (_, peer_ref) in &self.peer_list {
                if peer_ref.borrow().bootstraped() {
                    continue;
                }
                peer_ref.borrow_mut().set_bootstraped();
                let session_status = peer_ref.borrow().session.status();
                match session_status {
                    SessionStatus::Init => {
                        Self::notify_bootstrap(
                            self.protocol.clone(),
                            peer_ref.clone(),
                            &table
                        )
                    },
                    _ => {}
                };
            }
        }

        for (_, peer_ref) in &self.peer_list {
            Self::heartbeat(
                self.protocol.clone(),
                peer_ref.clone()
            )
        }

        println!("loop: {}, peer_list {:?}", self.eventloop.round, self.peer_list);
    }
}

impl Notify for P2PController {
    #[inline]
    fn notify_bootstrap(protocol: P2PProtocol, peer_ref: PeerRef, table: &PeerTable) {
        let result = peer_ref.borrow_mut().session.send_event(protocol.bootstrap(table));
        match result {
            Err(ref e) if e.kind() == ErrorKind::ConnectionAborted => {
                // EAGAIN
                peer_ref.borrow_mut().session.set_status(SessionStatus::Abort);
            },
            _ => {}
        }
    }

    #[inline]
    fn notify_gossip(protocol: P2PProtocol, peer_ref: PeerRef, table: &PeerTable, self_block_len: usize) {
        let result = peer_ref.borrow_mut().session.send_event(protocol.gossip(self_block_len, table));
        match result {
            Err(ref e) if e.kind() == ErrorKind::ConnectionAborted => {
                // EAGAIN
                peer_ref.borrow_mut().session.set_status(SessionStatus::Abort);
            },
            _ => {}
        }
    }

    #[inline]
    fn heartbeat(protocol: P2PProtocol, peer_ref: PeerRef) {
        let result = peer_ref.borrow_mut().session.send_event(protocol.heartbeat());
        match result {
            Err(ref e) if e.kind() == ErrorKind::ConnectionAborted => {
                // EAGAIN
                peer_ref.borrow_mut().session.set_status(SessionStatus::Abort);
            },
            _ => {}
        }
    }
}

impl Observe for P2PController {
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

impl Thread for P2PController {
    fn new(name: String) -> Result<Self> {
        let config = NetConfig::load();

        //TODO: make socket resuseable
        let server = TcpListener::bind(&config.server_addr());
        let account = Account::load();

        match (server, account) {
            (Ok(server), Some(account)) => {
                let mut peer_list = HashMap::<Token, PeerRef>::new();
                Ok(P2PController {
                    name: name,
                    account: account.clone(),
                    height: 0usize,
                    peer_list: peer_list,
                    min_required_peers: config.min_required_peer(),
                    max_allowed_peers: config.max_allowed_peers(),
                    waiting_list: vec![],
                    max_waiting_list: config.max_waitinglist_size(),
                    block_list: vec![],
                    max_blocked_peers: config.max_blocklist_size(),
                    eventloop: NetworkEventLoop::new(config.events_size()),
                    listener: server,
                    ch_pair: None,
                    last_updated: Utc::now(),
                    protocol: P2PProtocol::new(),
                    config: config
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

    /// # msg_handler(&mut self, 1)
    /// **Usage**
    /// - consume message from the inter-controller message channel,
    /// - tranform the inter-controller message into [[P2PMessage]]
    /// - send the [[P2PMessage]] by calling [[]]
    /// ## Examples
    /// ```
    /// ```
    fn msg_handler(&mut self, msg: Message) {
        match msg.msg.as_ref() {
            "gossip" => {
                let token = Token(msg.op as usize);
                let mut peer_ref = self.peer_list.get(&token);
                if let None = peer_ref {
                    return;
                }

                let peer_ref = peer_ref.unwrap().clone();

                // generate hosts list
                let hosts: Vec<String> = self.peer_list.iter()
                    .map(|pair| {
                        pair.1.borrow().addr().to_string()
                    }).collect();
                let table = PeerTable::new_with_hosts(hosts);
                Self::notify_gossip(
                    self.protocol.clone(),
                    peer_ref,
                    &table,
                    self.height
                );
            },
            _ => {}
        }
    }

    fn set_status(&mut self, status: ThreadStatus) {
        self.eventloop.status = status;
    }
}

impl Drop for P2PController {
    fn drop(&mut self) {

    }
}


#[cfg(test)]
mod p2p {
    use super::*;

    #[test]
    fn test_launch() {
        P2PController::launch_controller_with_channel("server");
        thread::sleep_ms(1000);
        assert!(MESSAGE_CENTER
            .lock()
            .unwrap()
            .channels_exist_by_name("server".to_string()));
    }

    #[test]
    fn test_start() {

    }
}