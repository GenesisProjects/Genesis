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
        let listener = TcpListener::bind(addr).unwrap();
        let poll = Poll::new().unwrap();
        let event_loop = EventLoop::new(poll);
        panic!("")
        /*if let Some(l) = listener {
            Some(MessageCenter { ip_addr: addr.to_owned(), event_loop: event_loop, listener: l })
        } else {
            None
        }*/

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
        let mut events = Events::with_capacity(1024);
        /*let handle = thread::spawn(|| {
            // Create storage for events

            // Start main loop
            loop {
                self.poll.poll(&mut events, None).unwrap();

                for event in events.iter() {

                }
            }
        });
        self.handle = Some(handle);*/
    }
}