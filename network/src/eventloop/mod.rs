//! [mio](https://carllerche.github.io/mio/mio/index.html) event loop
//!
//! Network module allow operation system select sockets those are ready for I/O.
//! Each round of loop are called **tick**.
//! Inside one tick, NetworkEventLoop will poll and refresh I/O events.
//! Objects implemented `Evented` could be registered in the NetworkEventLoop and obtain an auto-increase.
//!
use common::thread::ThreadStatus;
use mio::*;
use mio::net::TcpListener;

use std::io::*;
use std::sync::Mutex;
use std::time::Duration;
use std::marker::PhantomData;

/// The mio token for tcp connection listener
pub const SERVER_TOKEN: Token = Token(0);
/// The poll will give up cpu to let p2p controller update
pub const TIMEOUT_MILISECOND: u64 = 3000u64;

lazy_static! {
    static ref TOKEN_SEQ: Mutex<usize> = {
        Mutex::new(1usize)
    };
}

fn token_generator() -> Token {
    let mut seq = TOKEN_SEQ.lock().unwrap();
    let token = Token(*seq);
    *seq += 1;
    token
}

pub struct NetworkEventLoop<T> {
    pub events: Events,
    pub events_size: usize,
    pub round: usize,
    pub status: ThreadStatus,
    poll: Poll,
    phantom: PhantomData<T>
}

impl<T> NetworkEventLoop<T> where T: Evented {
    pub fn new(events_size: usize) -> Self {
        // Event storage
        let events = Events::with_capacity(events_size);
        // The [[Poll]] instance
        let poll = Poll::new().expect("Can not instantialize poll");

        NetworkEventLoop {
            round: 0usize,
            events: events,
            events_size: events_size,
            poll: poll,
            status: ThreadStatus::Stop,
            phantom: PhantomData
        }
    }

    /// Register a `TCPListener` instance with `SERVER_TOKEN`.
    /// `TCPListener` will bind to a port and listen to connection event.
    /// New connection will accept if event with `SERVER_TOKEN` has been triggered.
    ///
    /// # Example
    /// ```ignore
    /// // Init a listener
    /// let server = TcpListener::bind(&"127.0.0.1:19090".into());
    ///
    /// // Register the listener with a eventloop
    /// let mut eventloop: NetworkEventLoop<ObjectEvented> = NetworkEventLoop::new(1024);
    /// eventloop.register_server(&server);
    ///
    /// // Wait for connection
    /// for event in &(self.eventloop.events) {
    ///     match event.token() {
    ///         SERVER_TOKEN => {
    ///             println!("server event {:?}", event.token());
    ///             match self.listener.accept() {
    ///                 Ok((socket, addr)) => {
    ///                     println!("New socket has been accepted")
    ///                 }
    ///                 Err(e) => {
    ///                     println!("Could not accept the new connection.")
    ///                 }
    ///             }
    ///         }
    ///         _ => {
    ///             println!("Not a server event, ignore it.")
    ///         }
    ///     }
    /// }
    /// ```
    pub fn register_server(&self, listener: &TcpListener) {
        let new_token = SERVER_TOKEN;
        let _ = self.poll.register(listener, new_token, Ready::readable(), PollOpt::edge());
    }

    pub fn register_peer(&self, peer: &T) -> Result<(Token)> {
        let new_token = token_generator();
        match self.poll.register(peer, new_token, Ready::readable(), PollOpt::edge()) {
            Ok(_) => Ok(new_token),
            Err(e) => Err(e)
        }

    }

    pub fn reregister_peer(&self, token: Token, peer: &T) {
        let _ = self.poll.reregister(peer, token, Ready::readable(), PollOpt::edge());
    }

    pub fn deregister(&self, peer: &T) {
        let _ = self.poll.deregister(peer);
    }

    /// Fetch new I/O ready events from sockets registered in the eventloop.
    /// Return number of ready sockets
    /// ```ignore
    /// // Define an evented object.
    /// struct ObjectEvented;
    /// impl Evented for ObjectEvented {
    ///     fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
    ///         unimplemented!()
    ///     }
    ///
    ///     fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
    ///         unimplemented!()
    ///     }
    ///
    ///     fn deregister(&self, poll: &Poll) -> Result<()> {
    ///         unimplemented!()
    ///     }
    /// }
    ///
    /// // Register the evented object.
    /// let object = ObjectEvented;
    /// let mut eventloop: NetworkEventLoop<ObjectEvented> = NetworkEventLoop::new(1024);
    /// let token = eventloop.register_peer(&object).unwrap();
    ///
    /// // Do some I/O to `object`
    ///
    /// // Event loop fetch events
    /// loop {
    ///     eventloop.next_tick();
    ///     for event in eventloop.events {
    ///         // Do something here
    ///     }
    /// }
    ///
    /// ```
    pub fn next_tick(&mut self) -> Result<usize> {
        //TODO: make loop span configurable
        self.poll.poll(&mut self.events, Some(Duration::from_millis(TIMEOUT_MILISECOND))).and_then(|events_size| {
            self.round += 1;
            Ok(events_size)
        })
    }
}
