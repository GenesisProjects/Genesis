use chrono::*;
use nat::*;
use eventloop::*;

use common::address::Address as Account;
use gen_processor::*;
use socket::*;

use mio::*;
use mio::net::{TcpListener, TcpStream};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::*;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::net::*;
use std::str::FromStr;
use std::time::Duration;

/// P2PController
pub struct P2PController {
    name: String,
    thread_status: ThreadStatus,
    listener: TcpListener,
    receiver: Option<Receiver<Message>>,

    peer_map: HashMap<Token, PeerSocket>,
    waiting_list: Vec<(Token, PeerSocket)>,
    eventloop: NetworkEventLoop<PeerSocket>,
}

impl P2PController {
    fn new(name: String, server: TcpListener, events_size: usize) -> Self {
        P2PController {
            name: name,
            thread_status: ThreadStatus::Pause,
            listener: server,
            receiver: None,
            peer_map: HashMap::new(),
            waiting_list: Vec::new(),
            eventloop: NetworkEventLoop::new(events_size)
        }
    }

    pub fn create(
        name: String,
        server_socket: TcpListener,
        events_size: usize,
        stack_size: usize) -> ContextRef<Self> {
        let controller: P2PController = P2PController::new(name, server_socket, events_size);
        controller.launch(stack_size)
    }

    /// Connect
   pub fn connect(&mut self, addr: SocketInfo) -> Result<Token> {
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                let mut peer = PeerSocket::new(stream);
                // register peer
                self.register_peer(peer)
            },
            Err(e) => Err(e)
        }
    }

    /// Register peer
    pub fn register_peer(&mut self, mut peer: PeerSocket) -> Result<Token> {
        let result = self.eventloop.register_peer(&peer);
        if let Ok(token) = result {
            peer.set_token(token.clone());
            self.peer_map.insert(token.clone(), peer);
        }
        result
    }

    /// Deregister peer
    pub fn deregister_peer(&mut self, token: Token) -> Result<()> {
        if let Some(peer) = self.peer_map.remove(&token) {
            self.eventloop.deregister(&peer)
        } else {
            Ok(())
        }
    }

    /// Drop all peers
    pub fn clear_peers(&mut self) {
        self.peer_map = HashMap::<Token, PeerSocket>::new();
    }

    /// Drop all peers in the waiting list
    pub fn clear_waiting_list(&mut self) {
        self.waiting_list = Vec::new();
    }

    fn peer_ref(&self, token: Token) -> Option<&PeerSocket> {
        self.peer_map.get(&token)
    }

    fn peer_mut_ref(&mut self, token: Token) -> Option<&mut PeerSocket> {
        self.peer_map.get_mut(&token)
    }


    // process mio events
    fn process_events(&mut self) {
        let mut new_peers: Vec<(Token, PeerSocket)> = vec![];
        let ready_tokens = self.eventloop.ready_tokens();
        for token in ready_tokens {
            match token {
                // if is server listening event
                SERVER_TOKEN => {
                    // accept the inbound connection
                    match self.listener.accept() {
                        Ok((socket, addr)) => {
                            println!("Accepting a new peer...");
                            // init peer
                            let mut peer = PeerSocket::new(socket);
                            // register peer
                            if let Ok(token) = self.eventloop.register_peer(&mut peer) {
                                peer.set_token(token.clone());
                                new_peers.push((token, peer));
                            }
                        },
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            // EAGAIN
                            println!("Socket is not ready anymore, stop accepting");
                        },
                        e => {
                            panic!("{:?}", e)
                        }
                    }
                },
                peer_token => {
                    // process peer event
                    if let Some(ref mut peer) = self.peer_mut_ref(peer_token) {
                        let result = peer.receive_buffer();
                        match result {
                            Ok(_) => {},
                            Err(_) => {
                                self.eventloop.reregister_peer(peer_token.clone(), peer);
                            }
                        }
                    }
                }
            }
        }
        // put new sockets in the waiting list
        self.waiting_list.append(&mut new_peers);
    }
}

impl Processor for P2PController {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        "".to_string()
    }

    fn status(&self) -> ThreadStatus {
        self.thread_status.clone()
    }

    fn set_status(&mut self, status: ThreadStatus) {
        self.thread_status = status;
    }

    fn receiver(&self) -> &Option<Receiver<Message>> {
        &self.receiver
    }

    fn set_receiver(&mut self, recv: Receiver<Message>) {
        self.receiver = Some(recv)
    }

    fn handle_msg(&mut self, msg: Message) {
        unimplemented!()
    }

    fn exec(&mut self) -> bool {
        match self.eventloop.next_tick() {
            Ok(_size) => {
                self.process_events();
            },
            Err(e) => {
                panic!("exception: {:?}", e);
            }
        }
        true
    }

    fn pre_exec(&mut self) -> bool {
        true
    }
}

impl Drop for P2PController {
    fn drop(&mut self) {

    }
}


#[cfg(test)]
mod p2p {
    use super::*;

    #[test]
    fn test_launch() {

    }

    #[test]
    fn test_start() {

    }
}