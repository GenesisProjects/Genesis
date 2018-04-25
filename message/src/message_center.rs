use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::thread;
use std::time::Duration;
use std::net::SocketAddr;

pub struct MessageCenter {
    ip_addr: SocketAddr,
    event_loop: EventLoop,
    listener: TcpListener
}

impl MessageCenter {
    fn new(addr: &SocketAddr) -> Option<Self> {
        listener = TcpListener::bind(addr).unwrap_or(None);
        let poll = Poll::new().unwrap();
        let event_loop = event_loop::new(poll);
    }
}

struct EventLoop {
    poll: Poll,
    handle: Option<thread::JoinHandle<()>>
}

impl EventLoop {
    fn new(poll: Poll) -> Self {
        EventLoop { poll: poll, handle: None }
    }

    fn start(&mut self) {
        let handle = thread::spawn(|| {
            // Create storage for events
            let mut events = Events::with_capacity(1024);
            // Start main loop
            loop {
                poll.poll(&mut events, None).unwrap();

                for event in events.iter() {

                }
            }
        });
        self.handle = handle;
    }
}