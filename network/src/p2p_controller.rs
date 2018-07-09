use nat::*;
use network_eventloop::*;
use peer::*;
use message::protocol::*;
use session::*;
use utils::*;

use chrono::*;

use common::address::Address as Account;
use common::gen_message::*;
use common::thread::{Thread, ThreadStatus};
use common::observe::Observe;

use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::collections::HashMap;
use std::io::*;
use std::rc::{Rc, Weak};
use std::sync::{Mutex, Arc, Condvar};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub const UPDATE_TIMEBASE: i64 = 3000;
pub const CHANNEL_NAME: &'static str = "P2P_CONTROLLER";

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
    account: Account,
    peer_list: HashMap<Token, PeerRef>,
    min_required_peers: usize,
    max_allowed_peers: usize,
    waiting_list: Vec<SocketAddr>,
    max_waiting_list: usize,
    block_list: Vec<SocketAddr>,
    max_blocked_peers: usize,
    eventloop: NetworkEventLoop,
    listener: TcpListener,
    ch_pair: Option<Arc<(Mutex<MessageChannel>, Condvar)>>,
    last_updated: DateTime<Utc>,
    protocol: P2PProtocol
}

impl P2PController {
    /// # launch_controller(1)
    /// **Usage**
    /// - launch the controller with a new thread
    /// - subscribe a interthread channel
    /// **Parameters**
    /// - 1. ***String(name)***: the interthread channel name
    /// ## Examples
    /// ```
    /// ```
    pub fn launch_controller() {
        P2PController::launch::<P2PController>(CHANNEL_NAME.to_string());
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
        //TODO: port configuable
        match get_local_ip() {
            Some(socket_info) => {
                match get_public_ip_addr(
                    Protocol::UPNP,
                    &(SocketAddr::new(socket_info, 19999), 19999)
                ) {
                    Some(socket_info) => {
                        match TcpStream::connect(&addr.0) {
                            Ok(stream) => {
                                Ok(Rc::new(Peer::new(stream, &addr.0)))
                            },
                            Err(e) => Err(e)
                        }
                    },
                    None => {
                        Err(Error::new(
                            ErrorKind::ConnectionRefused,
                            "Connot get a public interface"
                        ))
                    }
                }
            }
            None => Err(Error::new(
                ErrorKind::Other,
                "Connot get a local ip"
            ))
        }
    }

    fn init_peers_table(&mut self) {
        self.peer_list = HashMap::<Token, PeerRef>::new();
    }

    fn search_peers(&self) -> Vec<(Option<Account>, SocketInfo)> {
        let mut raw_peers_table = self.peer_list.values().map(|peer_ref| {
            peer_ref.peer_table()
        }).fold(Vec::<(Option<Account>, SocketInfo)>::new(), |mut init, ref mut table: Vec<(Option<Account>,SocketInfo)>| {
            init.append(table);
            init
        });

        // filter out identical elements
        raw_peers_table.sort_by(|&(ref addr_a, _), &(ref addr_b, _)| addr_a.partial_cmp(addr_b).unwrap());
        raw_peers_table.dedup_by(|&mut (ref addr_a, _), &mut (ref addr_b, _)| *addr_a == *addr_b);

        // filter out self
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref addr, _)| {
            if let Some(ref account) = (*addr) {
                account.clone() != self.account
            } else {
                true
            }
        }).collect();

        // filter out in current peer list
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref account, (ref addr, ref port))| !self.socket_exist(addr)).collect();

        // filter out in block list
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref account, (ref addr, ref port))| !self.socket_blocked(addr)).collect();

        raw_peers_table
    }

    fn socket_exist(&self, addr: &SocketAddr) -> bool {
        match self.peer_list.iter().find(|&(token, peer_ref)| {
            peer_ref.addr() == *addr
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
        self.waiting_list = self.search_peers().into_iter().map(|(ref account, (ref addr, ref port))| {
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
        self.peer_list.remove(&token);
    }

    fn ban_peer(&mut self, addr: &SocketAddr, loops: usize) {
        while self.block_list.len() > self.max_blocked_peers {
            self.block_list.remove(0);
        }
        self.block_list.push(addr.clone());
    }

    fn process_events(&mut self) {
        let mut new_peers: Vec<(Token, PeerRef)> = vec![];

        for event in &(self.eventloop.events) {
            match event.token() {
                SERVER_TOKEN => {
                    match self.listener.accept() {
                        Ok((socket, addr)) => {
                            // init peer
                            let peer = Peer::new(socket, &addr);
                            if !self.socket_exist(&addr) {
                                let token = self.eventloop.register_peer(&peer);
                                new_peers.push((token, Rc::new(peer)));
                            }
                        },
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            // EAGAIN
                        },
                        e => {
                            panic!("{:?}", e)
                        }
                    }
                },
                PEER_TOKEN => {
                    // process peer event
                    self.get_peer(PEER_TOKEN).and_then(|ref mut peer_ref| {
                        Rc::get_mut(peer_ref).unwrap().process();
                        Some(true)
                    });
                }
            }
        }
        for &(ref token, ref peer) in &new_peers {
            self.add_peer(token, peer.clone());
        }
    }
}

impl Notify for P2PController {
    fn notify_bootstrap(protocol: P2PProtocol, mut peer_ref: PeerRef, table: &PeerTable) {
        let peer = Rc::get_mut(&mut peer_ref).unwrap();
        peer.session.send_event(protocol.bootstrap(table));
    }

    fn notify_gossip(protocol: P2PProtocol, mut peer_ref: PeerRef, table: &PeerTable) {
        let peer = Rc::get_mut(&mut peer_ref).unwrap();
        peer.session.send_event(protocol.gossip(table));
    }
}

impl Observe for P2PController {
    fn subscribe(&mut self) {
        self.ch_pair = Some(
            MESSAGE_CENTER
            .lock()
            .unwrap()
            .subscribe(&CHANNEL_NAME.to_string())
            .clone()
        );
    }

    fn unsubscribe(&mut self) {
        if let Some(ch_pair) = self.ch_pair.clone() {
            let uid = (*ch_pair).0.lock().unwrap().uid.clone();
            self.ch_pair = None;
            MESSAGE_CENTER
                .lock()
                .unwrap()
                .unsubscribe(&"P2P_CONTROLLER".to_string(), uid);
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
    fn new() -> Result<Self> {
        //TODO: load port from config
        let addr = "127.0.0.1:19999".parse().unwrap();
        //TODO: make socket resuseable
        let server = TcpListener::bind(&addr);
        let account = Account::load();

        match (server, account) {
            (Ok(server), Some(account)) => {
                //TODO: load events size from config
                let event_loop = NetworkEventLoop::new(1024);
                //TODO: max_allowed_peers configuable
                let max_allowed_peers: usize = 512;
                //TODO: max_blocked_peers configuable
                let max_blocked_peers: usize = 1024;
                //TODO: max_waiting_list configuable
                let max_waiting_list: usize = 1024;
                //TODO: max_waiting_list configuable
                let min_required_peers: usize = 4;

                let mut peer_list = HashMap::<Token, PeerRef>::new();
                Ok(P2PController {
                    account: account.clone(),
                    peer_list: peer_list,
                    min_required_peers: min_required_peers,
                    max_allowed_peers: max_allowed_peers,
                    waiting_list: vec![],
                    max_waiting_list: max_waiting_list,
                    block_list: vec![],
                    max_blocked_peers: max_blocked_peers,
                    eventloop: event_loop,
                    listener: server,
                    ch_pair: None,
                    last_updated: Utc::now(),
                    protocol: P2PProtocol::new()
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
        // fetch the next tick
        let result = self.eventloop.next_tick();
        match self.eventloop.status {
            ThreadStatus::Running => {
                match result {
                    Ok(size) => {
                        // println!("{} events are ready", size);
                        self.process_events();
                        true
                    },
                    Err(e) => {
                        panic!("exception: {:?}", e);
                        false
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
                let peer_ref = self.peer_list.get(&token);
                if let None = peer_ref {
                    return;
                }

                // generate hosts list
                let hosts: Vec<(String, i32)> = self.peer_list.iter()
                    .map(|mut pair| {
                        (
                            pair.1.addr().to_string(),
                            39999 as i32
                        )
                    }).collect();
                let table = PeerTable::new_with_hosts(hosts);
                Self::notify_gossip(
                    self.protocol.clone(),
                    peer_ref.unwrap().clone(),
                    &table
                );
            },
            _ => {}
        }
    }

    fn set_status(&mut self, status: ThreadStatus) {
        self.eventloop.status = status;
    }

    /// # update(&mut self, 0)
    /// **Usage**
    /// - check peerlist, block invalid peers
    /// - refresh the waiting list if peers are not enough
    /// ## Examples
    /// ```
    /// ```
    fn update(&mut self) {
        if (Utc::now() - self.last_updated).num_milliseconds() < UPDATE_TIMEBASE {
            return;
        }
        self.last_updated = Utc::now();

        // find failed tokens in the peer list
        let failed_tokens: Vec<Token> = self.peer_list.iter().filter(|pair| {
            pair.1.credit() == 0
        }).map(|pair| {
            pair.0.clone()
        }).collect();

        // remove all failed tokens from the peer list
        for token in failed_tokens {
            let addr = self.get_peer(token.clone()).unwrap().addr().clone();
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
            let peers: Vec<PeerRef> = sockets.into_iter()
                .map(|addr| {
                    self.connect((addr, 39999))
                })
                .filter(|result| {
                    match result {
                        &Ok(_) => true,
                        &Err(_) => false
                    }
                })
                .map(|result| {
                    result.unwrap()
                })
                .collect();

            // register new peers to the eventloop, add into peer list
            for mut peer in peers {
                let token = self.eventloop.register_peer(&peer);
                Rc::get_mut(&mut peer.clone()).unwrap().set_token(token.clone());
                self.peer_list.insert(token, peer);
            }

            // bootstrap all peers at init status
            let hosts: Vec<(String, i32)> = self.peer_list.iter()
                .map(|mut pair| {
                    (
                        pair.1.addr().to_string(),
                        39999 as i32
                    )
                }).collect();
            let table = PeerTable::new_with_hosts(hosts);
            for (_, peer_ref) in &self.peer_list {
                match peer_ref.clone().session.status() {
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
    }
}

impl Drop for P2PController {
    fn drop(&mut self) {
        unimplemented!()
    }
}