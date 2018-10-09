use chrono::*;
use nat::*;
use eventloop::*;

use common::address::Address as Account;
use gen_processor::*;
use socket::*;

use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::*;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::net::*;
use std::str::FromStr;
use std::time::Duration;

/// P2PController
pub struct P2PController {
    name: String,
    thread_status: ThreadStatus,
    listener: TcpListener,
    receiver: Option<Receiver<Message>>,

    peer_list: HashMap<Token, PeerRef>,
    min_required_peers: usize,
    max_allowed_peers: usize,

    waiting_list: Vec<SocketAddr>,
    max_waiting_list: usize,

    block_list: Vec<SocketAddr>,
    max_blocked_peers: usize,

    eventloop: NetworkEventLoop<Peer>,
    last_updated: DateTime<Utc>,
    protocol: P2PProtocol,
}

impl P2PController {
    /// Launch_controller
    pub fn launch_controller() -> ContextRef<Self> {
        //P2PController::launch::<P2PController>("P2PController".to_string());
    }

    /// Connect
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
        self.peer_list.insert(token.clone(), peer_ref.clone());
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

    // process mio events
    fn process_events(&mut self) {
        let mut new_peers: Vec<(Token, PeerRef)> = vec![];

        for event in &(self.eventloop.events) {
            match event.token() {
                // if is server listening event
                SERVER_TOKEN => {
                    // accept the inbound connection
                    match self.listener.accept() {
                        Ok((socket, addr)) => {
                            println!("Accepting a new peer...");
                            // if the socket has been blocked or existed
                            if !self.socket_exist(&addr) && !self.has_blocked(&addr) {
                                // init peer
                                let mut peer = PeerSocket::new(socket);
                                // register peer
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

    /// Update
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

impl Processor for P2PController {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        "".to_string()
    }

    fn status(&self) -> ThreadStatus {
        self.thread_status.clone()
    }

    fn set_status(&mut self, status: ThreadStatus) {
        self.thread_status = status;
    }

    fn receiver(&self) -> &Option<Receiver<Message>> {
        &self.receiver
    }

    fn set_receiver(&mut self, recv: Receiver<Message>) {
        self.receiver = Some(recv)
    }

    fn handle_msg(&mut self, msg: Message) {
        unimplemented!()
    }

    fn exec(&mut self) -> bool {
        match self.eventloop.next_tick() {
            Ok(_size) => {
                self.process_events();
                true
            },
            Err(e) => {
                panic!("exception: {:?}", e);
            }
        }
        self.update();
        true
    }

    fn pre_exec(&mut self) -> bool {
        true
    }
}


impl Thread for P2PController {
    fn new(name: String) -> Result<Self> {
        let config = P2PConfig::load();

        //TODO: make socket resuseable
        let server = TcpListener::bind(&config.server_addr());

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

    fn get_status(&self) -> ThreadStatus {
        self.eventloop.status
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