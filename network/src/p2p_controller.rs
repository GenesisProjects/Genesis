use peer::*;
use session::*;

use std::collections::HashMap;
use std::io::*;
use std::sync::Mutex;

use mio::*;
use mio::net::{TcpListener, TcpStream};

use common::address::Address;

lazy_static! {
    pub static ref TOKEN_SEQ: Mutex<usize> = {
        Mutex::new(0usize)
    };
}

fn token_generator() -> Token {
    let mut seq = TOKEN_SEQ.lock().unwrap();
    let token = Token(*seq);
    *seq += 1;
    token
}

struct EventLoop {
    events: Events,
    poll: Poll
}

impl EventLoop {
    pub fn new(events_size: usize) -> Self {
        // Event storage
        let mut events = Events::with_capacity(events_size);
        // The `Poll` instance
        let poll = Poll::new().expect("Can not instantialize poll");

        EventLoop {
            events: events,
            poll: poll
        }
    }

    pub fn register(&self, peer: &Peer) -> Token {
        let new_token = token_generator();
        self.poll.register(peer, new_token, Ready::readable(), PollOpt::level());
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
            Ok(events_size)
        });

        unimplemented!()
    }
}

pub struct P2PController {
    loop_count: usize,
    peer_list: HashMap<Token, (PeerRef, usize)>,
    black_list: Vec<(Address, usize)>,
    eventloop: EventLoop,
    listener: Option<TcpListener>,
}

impl P2PController {

    fn new() -> Self {
        unimplemented!()
    }

    fn init(&mut self, event_size: usize) {

    }

    fn search_peers(&self) -> Vec<PeerRef> {
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

    fn ban_peer(addr: Address, loops: usize) {
        unimplemented!()
    }

    fn connect_peer<'a>(peer_ref: PeerRef, local_block_info: &'a BlockInfo) -> Result<BlockInfo> {
        unimplemented!()
    }


    fn start_run_loop(&mut self) {
        loop {
            match self.eventloop.process() {
                Ok(events_size) => {
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