use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::thread;
use std::sync::Arc;
use std::time::Duration;
use std::net::SocketAddr;

pub struct MessageCenter {
    ip_addr: SocketAddr,
    event_loop: EventLoop,
    listener: TcpListener
}

impl MessageCenter {
    fn new(addr: &SocketAddr) -> Option<Self> {
        let listener = match TcpListener::bind(addr) {
            Ok(r) => r,
            Err(e) => panic!("Can not bind to the socket: {:?}", addr)
        };

        let event_loop = EventLoop::new();

        Some(MessageCenter { ip_addr: addr.to_owned(), event_loop: event_loop, listener: listener })
    }
}

struct EventLoop {
    handle: Option<thread::JoinHandle<()>>,
    poll_arc_pointer: Option<Arc<Poll>>
}

impl EventLoop {
    fn new() -> Self {
        EventLoop { handle: None, poll_arc_pointer: None }
    }

    fn start(&mut self) {
        let mut events: Events = Events::with_capacity(1024);
        let poll = Poll::new().unwrap();
        // Retain a thread safe pointer into the current thread eventloop instant.
        let poll_arc_pointer = Arc::new(poll);
        // Retain a thread safe pointer into the runloop thread
        let thread_poll_arc_pointer = poll_arc_pointer.clone();
        self.poll_arc_pointer = Some(poll_arc_pointer.clone());

        // Create storage for events
        let handle = thread::spawn(move|| {
            // Start main loop
            loop {
                // Poll the events
                thread_poll_arc_pointer.poll(&mut events, None).unwrap();
                // Dispatch events into handles
                for event in events.iter() {

                }
            }
        });
        self.handle = Some(handle);
    }
}