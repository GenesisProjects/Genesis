use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::thread;
use std::sync::Arc;
use std::time::Duration;
use std::net::SocketAddr;

use gen_utils::config_parser::SETTINGS;

pub struct MessageCenter {
    ip_addr: SocketAddr,
    event_loop: EventLoop,
    listener: TcpListener,
    emitter: TcpStream
}

impl MessageCenter {
    fn new(addr: &SocketAddr) -> Option<Self> {
        let listener = match TcpListener::bind(addr) {
            Ok(r) => r,
            Err(e) => panic!("Can not bind to the socket: {:?}", addr)
        };

        let emitter = match TcpStream::connect(addr) {
            Ok(r) => r,
            Err(e) => panic!("Can not bind to the connect: {:?}", addr)
        };

        let event_loop = EventLoop::new();

        Some(MessageCenter { ip_addr: addr.to_owned(), event_loop: event_loop, listener: listener, emitter: emitter })
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

    fn register(&self, token: Token, listener: &TcpListener) {
        // Start listening for incoming connections
        if let Some(poll_pointer) = self.poll_arc_pointer.to_owned() {
            poll_pointer.register(listener, token, Ready::readable(), PollOpt::edge()).unwrap();
        }
    }

    fn start(&mut self) {
        // Create events queue
        let mut events: Events = Events::with_capacity(SETTINGS.read().unwrap().get_int("event_queue_size").unwrap() as usize);
        let poll = Poll::new().unwrap();
        // Retain a thread safe pointer into the current thread eventloop instant.
        let poll_arc_pointer = Arc::new(poll);
        // Retain a thread safe pointer into the runloop thread
        let thread_poll_arc_pointer = poll_arc_pointer.clone();
        self.poll_arc_pointer = Some(poll_arc_pointer.clone());

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