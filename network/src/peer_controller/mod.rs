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

pub const BAN_LIST_LIMIT: usize     = 1024;
pub const PEER_MAP_LIMIT: usize     = 1024;
pub const WAITING_LIST_LIMIT: usize = 1024;


/// P2PController
pub struct P2PController {
    name: String,
    thread_status: ThreadStatus,
    listener: TcpListener,
    receiver: Option<Receiver<Message>>,

    peer_map: HashMap<Token, PeerSocket>,
    ban_list: Vec<SocketAddr>,
    waiting_list: Vec<(SocketAddr, PeerSocket)>,
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
            ban_list: Vec::new(),
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
        if self.peer_map.len() >= PEER_MAP_LIMIT {
            return Err(Error::new(ErrorKind::Other, "peer map is over limited"));
        }
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
    fn clear_peer_map(&mut self) {
        self.peer_map = HashMap::<Token, PeerSocket>::new();
    }

    /// Existed in peer map or not
    fn existed_in_peer_map(&self, addr: &SocketAddr) -> bool {
        self.peer_map.iter().any(|(key, val)| {
            val.addr() == *addr
        })
    }

    /// Append waiting list
    fn append_waiting_list(&mut self, new_peers: &mut Vec<(SocketAddr, PeerSocket)>) {
        let remain_size = WAITING_LIST_LIMIT - self.waiting_list.len();
        let end_pos = if remain_size > self.waiting_list.len() {
            self.waiting_list.len()
        } else {
            remain_size
        };
        let mut target_peers: Vec<(SocketAddr, PeerSocket)> = new_peers.drain(0..end_pos).collect();
        self.waiting_list.append(&mut target_peers);
    }

    /// Drop all peers in the waiting list
    fn clear_waiting_list(&mut self) {
        self.waiting_list = Vec::new();
    }

    /// Existed in waiting list or not
    fn existed_in_waiting_list(&self, addr: &SocketAddr) -> bool {
        self.waiting_list.iter().any(|pair| {
            pair.0 == *addr
        })
    }

    /// Ban a peer
    pub fn ban(&mut self, addr: &SocketAddr) {
       if !self.existed_in_ban_list(addr) {
            self.ban_list.push(addr.to_owned())
       }
    }

    /// Existed in ban list or not
    fn existed_in_ban_list(&self, addr: &SocketAddr) -> bool {
        self.ban_list.iter().any(|&e| {
            e == *addr
        })
    }

    fn peer_ref(&self, token: Token) -> Option<&PeerSocket> {
        self.peer_map.get(&token)
    }

    fn peer_mut_ref(&mut self, token: Token) -> Option<&mut PeerSocket> {
        self.peer_map.get_mut(&token)
    }


    // process mio events
    fn process_events(&mut self) {
        let mut new_peers: Vec<(SocketAddr, PeerSocket)> = vec![];
        let ready_tokens = self.eventloop.ready_tokens();
        for token in ready_tokens {
            match token {
                // if is server listening event
                SERVER_TOKEN => {
                    // accept the inbound connection
                    match self.listener.accept() {
                        Ok((socket, addr)) => {
                            // init peer
                            let mut peer = PeerSocket::new(socket);
                            let peer_addr = peer.addr();
                            // push to the waiting list if available
                            if !self.existed_in_waiting_list(&peer_addr)
                                && !self.existed_in_peer_map(&peer_addr)
                                && !self.existed_in_ban_list(&peer_addr) {
                                new_peers.push((peer.addr(), peer));
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
        // put new peers in the waiting list
        self.append_waiting_list(&mut new_peers);
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