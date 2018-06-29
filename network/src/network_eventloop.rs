use peer::*;
use session::*;
use utils::*;

use std::io::*;
use std::time::Duration;
use std::sync::{Mutex, Arc, Condvar};

use mio::*;
use mio::net::{TcpListener, TcpStream};

use common::thread::{Thread, ThreadStatus};

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

pub struct NetworkEventLoop {
    pub events: Events,
    pub loop_count: usize,
    pub status: ThreadStatus,
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
            poll: poll,
            status: ThreadStatus::Stop
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

    pub fn next_tick(&mut self) -> Result<usize> {
        //TODO: make loop span configurable
        self.poll.poll(&mut self.events, Some(Duration::from_millis(10))).and_then(|events_size| {
            self.loop_count += 1;
            Ok(events_size)
        })
    }
}
