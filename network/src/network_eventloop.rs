use common::thread::{Thread, ThreadStatus};
use mio::*;
use mio::net::{Poll, TcpListener, TcpStream};
use peer::*;
use session::*;
use std::io::*;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
use utils::*;

const SERVER_TOKEN: Token = Token(0);

lazy_static! {
    pub static ref TOKEN_SEQ: Mutex<usize> = {
        Mutex::new(1usize)
    };
}

/// # token_generator(0)
/// **Usage**
/// - generate a unique token
/// **Return**
/// - 1. ***[[Token]]***
/// ## Examples
/// ```
/// let new_token = token_generator();
/// ```
fn token_generator() -> Token {
    let mut seq = TOKEN_SEQ.lock().unwrap();
    let token = Token(*seq);
    *seq += 1;
    token
}

/// # NetworkEventLoop
/// **Usage**
/// - a implementation of the mio, polled by [[common::thread::Thread]]
/// **Parameters**
/// - 1. ***events***       events queue
/// - 2. ***round***        current round
/// - 3. ***status***       instance of [[ThreadStatus]]
/// - 4. ***poll***         instance of [[Poll]]
/// ## Examples
/// ```
/// ```
pub struct NetworkEventLoop {
    pub events: Events,
    pub round: usize,
    pub status: ThreadStatus,
    poll: Poll,
}

impl NetworkEventLoop {
    pub fn new(events_size: usize) -> Self {
        // Event storage
        let mut events = Events::with_capacity(events_size);
        // The [[Poll]] instance
        let poll = Poll::new().expect("Can not instantialize poll");

        NetworkEventLoop {
            round: 0usize,
            events: events,
            poll: poll,
            status: ThreadStatus::Stop,
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

    /// # next_tick(&mut self)
    /// **Usage**
    /// - fetch new events, called by a thread at beginning of each loop-cycle.
    /// **Result**
    /// Result<usize>: num of new events
    /// ## Examples
    /// ```
    /// ```
    pub fn next_tick(&mut self) -> Result<usize> {
        //TODO: make loop span configurable
        self.poll.poll(&mut self.events, Some(Duration::from_millis(10))).and_then(|events_size| {
            self.round += 1;
            Ok(events_size)
        })
    }
}
