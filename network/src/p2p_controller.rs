use peer::*;
use session::*;
use utils::*;

use std::collections::HashMap;
use std::io::*;
use std::sync::Mutex;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;
use nat::*;

use mio::*;
use mio::net::{TcpListener, TcpStream};

const SERVER_TOKEN: Token = Token(0);

lazy_static! {
    pub static ref TOKEN_SEQ: Mutex<usize> = {
        Mutex::new(1usize)
    };
}

fn token_generator() -> Token {
    let mut seq = TOKEN_SEQ.lock().unwrap();
    let token = Token(*seq);
    *seq += 1;
    token
}

struct NetworkEventLoop {
    loop_count: usize,
    events: Events,
    poll: Poll
}

impl NetworkEventLoop {

    pub fn new(events_size: usize) -> Self {
        // Event storage
        let mut events = Events::with_capacity(events_size);
        // The `Poll` instance
        let poll = Poll::new().expect("Can not instantialize poll");

        NetworkEventLoop {
            loop_count: 0usize,
            events: events,
            poll: poll
        }
    }

    pub fn register_server(&self, listener: &TcpListener) {
        let new_token = SERVER_TOKEN;
        self.poll.register(listener, new_token, Ready::readable(), PollOpt::edge());
    }

    pub fn register_peer(&self, peer: &Peer) -> Token {
        let new_token = token_generator();
        self.poll.register(peer, new_token, Ready::readable(), PollOpt::edge());
        new_token
    }

    pub fn deregister(&self, peer: &Peer) {
        self.poll.deregister(peer);
    }

    pub fn process(&mut self) -> Result<usize> {
        self.poll.poll(&mut self.events, None).and_then(|events_size| {
            for event in &self.events {
                let token = event.token();
            }
            self.loop_count += 1;
            Ok(events_size)
        });

        unimplemented!()
    }
}

pub struct P2PController {
    account: Account,
    peer_list: HashMap<Token, PeerRef>,
    max_allowed_peers: usize,
    waitting_list: Vec<SocketAddr>,
    max_waiting_list: usize,
    block_list: Vec<SocketAddr>,
    max_blocked_peers: usize,
    eventloop: NetworkEventLoop,
    listener: TcpListener,
}

impl P2PController {

    pub fn new(account: &Account) -> Self {
        //TODO: load port from config
        let addr = "127.0.0.1:39999".parse().unwrap();
        let server = TcpListener::bind(&addr).unwrap();
        //TODO: load events size from config
        let event_loop = NetworkEventLoop::new(1024);
        //TODO: max_allowed_peers configuable
        let max_allowed_peers = 512;
        //TODO: max_blocked_peers configuable
        let max_blocked_peers = 1024;
        //TODO: max_waiting_list configuable
        let max_waiting_list = 1024;

        let mut peer_list = HashMap::<Token, PeerRef>::new();
        P2PController {
            account: account.clone(),
            peer_list: peer_list,
            max_allowed_peers: max_allowed_peers,
            waitting_list: vec![],
            max_waiting_list: max_waiting_list,
            block_list: vec![],
            max_blocked_peers: max_blocked_peers,
            eventloop: event_loop,
            listener: server,
        }
    }

    pub fn bootstrap(&mut self) {
        //TODO: port configuable
        let socket_info = match get_local_ip() {
            Some(socket_info) => get_public_ip_addr(Protocol::UPNP, &(SocketAddr::new(socket_info, 19999), 19999)),
            None => None
        };

        self.init_peers_table();

        unimplemented!()
    }

    fn init_peers_table(&mut self) {
        unimplemented!()
    }

    fn search_peers(&self) -> Vec<(Account, SocketInfo)> {
        let mut raw_peers_table = self.peer_list.values().map(|peer_ref| {
            peer_ref.peer_table()
        }).fold(Vec::<(Account, SocketInfo)>::new(), |mut init, ref mut table: Vec<(Account,SocketInfo)>| {
            init.append(table);
            init
        });

        // filter out identical elements
        raw_peers_table.sort_by(|&(ref addr_a, _), &(ref addr_b, _)| addr_a.partial_cmp(addr_b).unwrap());
        raw_peers_table.dedup_by(|&mut (ref addr_a, _), &mut (ref addr_b, _)| *addr_a == *addr_b);

        // filter out self
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref addr, _)| *addr != self.account).collect();

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
        unimplemented!()
    }

    fn reflesh_peer_list(&mut self) {
        unimplemented!()
    }

    fn add_peer(peer_ref: PeerRef) {
        unimplemented!()
    }

    fn remove_peer(peer_ref: PeerRef) {
        unimplemented!()
    }

    fn ban_peer(addr: Account, loops: usize) {
        unimplemented!()
    }

    fn register<'a>(&self, peer_ref: PeerRef, local_block_info: &'a BlockInfo) -> Result<BlockInfo> {
        unimplemented!()
    }


    fn start_run_loop(&mut self) {
        loop {
            match self.eventloop.process() {
                Ok(events_size) => {
                    for event in &(self.eventloop.events) {
                        match event.token() {
                            SERVER_TOKEN => {
                                match self.listener.accept() {
                                    Ok((socket, _)) => {

                                    },
                                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                                        // EAGAIN
                                    },
                                    e => {

                                    }
                                }
                            },
                            peer_token => {

                            }
                        }
                    }
                    unimplemented!()
                },
                Err(e) => {
                    break;
                    unimplemented!()
                }
            }
        }
    }
}