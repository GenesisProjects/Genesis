use peer::*;
use session::*;

use std::collections::HashMap;
use std::io::*;
use std::sync::Mutex;

use mio::*;
use mio::net::{TcpListener, TcpStream};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;

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
    peer_list: HashMap<Token, PeerRef>,
    black_list: Vec<Account>,
    eventloop: NetworkEventLoop,
    listener: TcpListener,
}

impl P2PController {

    fn new() -> Self {
        //TODO: load port from config
        let addr = "127.0.0.1:39999".parse().unwrap();
        let server = TcpListener::bind(&addr).unwrap();
        //TODO: load events size from config
        let event_loop = NetworkEventLoop::new(1024);
        let mut peer_list = HashMap::<Token, PeerRef>::new();
        P2PController {
            peer_list: peer_list,
            black_list: vec![],
            eventloop: event_loop,
            listener: server,
        }
    }

    fn init(&mut self, event_size: usize) {

    }

    fn search_peers(&self) -> Vec<SocketAddr> {
        let peer_tables = self.peer_list.iter().map(|(token, peer_ref)| {
            (token.clone(), peer_ref.peer_table())
        }).collect();

        unimplemented!()
    }

    fn default_peers(&self) -> Vec<PeerRef> {
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

    fn connect_peer<'a>(peer_ref: PeerRef, local_block_info: &'a BlockInfo) -> Result<BlockInfo> {
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