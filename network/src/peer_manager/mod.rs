use chrono::*;
use nat::*;
use eventloop::*;

use common::address::Address as Account;
use gen_processor::*;
use socket::*;
use socket::message::defines::*;

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

const TIME_SPAN: u64 = 100;

pub trait SocketMessageListener: Send {
    fn notify(&mut self, msg: SocketMessage);
}

/// P2PController
pub struct P2PManager {
    name: String,
    thread_status: ThreadStatus,
    listener: TcpListener,
    receiver: Option<Receiver<Message>>,

    peer_map: HashMap<Token, PeerSocket>,
    ban_list: Vec<SocketAddr>,
    waiting_list: Vec<(SocketAddr, PeerSocket)>,
    eventloop: NetworkEventLoop<PeerSocket>,

    peer_map_limit: usize,
    ban_list_limit: usize,
    waiting_list_limit: usize,

    msg_listener: ContextRef<SocketMessageListener>
}

impl P2PManager {
    fn new(
        name: String,
        listener_addr: SocketAddr,
        events_size: usize,
        peer_map_limit: usize,
        ban_list_limit: usize,
        waiting_list_limit: usize,
        msg_listener: ContextRef<SocketMessageListener>
    ) -> Result<Self> {
        match TcpListener::bind(&listener_addr) {
            Ok(listerner) => Ok(P2PManager {
                name: name,
                thread_status: ThreadStatus::Pause,
                listener: listerner,
                receiver: None,
                peer_map: HashMap::new(),
                ban_list: Vec::new(),
                waiting_list: Vec::new(),
                eventloop: NetworkEventLoop::new(events_size),
                peer_map_limit: peer_map_limit,
                ban_list_limit: ban_list_limit,
                waiting_list_limit: waiting_list_limit,
                msg_listener: msg_listener
            }),
            Err(e) => Err(e)
        }
    }

    /// Create a P2PController.
    /// Return contextref.
    pub fn create(
        name: String,
        listener_addr: SocketAddr,
        events_size: usize,
        stack_size: usize,
        peer_map_limit: usize,
        ban_list_limit: usize,
        waiting_list_limit: usize,
        msg_listener: ContextRef<SocketMessageListener>
    ) -> Result<ContextRef<Self>> {
        P2PManager::new(
            name,
            listener_addr,
            events_size,
            peer_map_limit,
            ban_list_limit,
            waiting_list_limit,
            msg_listener
        ).and_then(|controller| {
            Ok(controller.launch(stack_size))
        })
    }

    /// Connect
   pub fn connect(&mut self, addr: SocketAddr) -> Result<Token> {
        if !self.addr_is_valid(&addr) {
            return Err(Error::new(ErrorKind::Other, "the socket address is not valid."));
        }
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                let mut peer = PeerSocket::new(stream);
                // register peer
                self.accept_peer(peer)
            },
            Err(e) => Err(e)
        }
    }

    /// Accept and register peer
    pub fn accept_peer(&mut self, mut peer: PeerSocket) -> Result<Token> {
        if self.peer_map.len() >= self.peer_map_limit {
            return Err(Error::new(ErrorKind::Other, "peer map is over limited"));
        }
        let result = self.eventloop.register_peer(&peer);
        if let Ok(token) = result {
            peer.set_token(token.clone());
            self.peer_map.insert(token.clone(), peer);
        }
        result
    }

    /// Deregister and drop peer
    pub fn drop_peer(&mut self, token: Token) -> Result<()> {
        if let Some(peer) = self.peer_map.remove(&token) {
            self.eventloop.deregister(&peer)
        } else {
            Ok(())
        }
    }

    /// If the peer is alive or not
    pub fn peer_is_alive(&self, token: Token) -> bool {
        self.peer_ref(token).is_alive()
    }

    /// Peer socket address
    pub fn peer_addr(&self, token: Token) -> SocketAddr {
        self.peer_ref(token).addr()
    }

    /// Drop all peers
    fn clear_peer_map(&mut self) {
        let tokens: Vec<Token> = self.peer_map.iter().map(|(token, peer)| {
            token.clone()
        }).collect();
        for token in tokens {
            self.drop_peer(token);
        }
    }

    /// Existed in peer map or not
    fn existed_in_peer_map(&self, addr: &SocketAddr) -> bool {
        self.peer_map.iter().any(|(token, peer)| {
            peer.addr() == *addr
        })
    }

    /// Append waiting list
    fn append_waiting_list(&mut self, new_peers: &mut Vec<(SocketAddr, PeerSocket)>) {
        let remain_size = self.waiting_list_limit - self.waiting_list.len();
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

    /// Unban a peer
    pub fn unban(&mut self, addr: &SocketAddr) {
        let index_option = self.ban_list.iter().position(|x| x == addr);
        match index_option {
            Some(index) => { self.ban_list.remove(index); }
            _ => {}
        }
    }

    /// Existed in ban list or not
    fn existed_in_ban_list(&self, addr: &SocketAddr) -> bool {
        self.ban_list.iter().any(|&e| {
            e == *addr
        })
    }

    /// Send message
    pub fn send_msg(&mut self, token: Token, msg: SocketMessage) -> Result<()> {
        self.peer_mut_ref(token).write_msg(msg)
    }

    fn peer_ref(&self, token: Token) -> &PeerSocket {
        self.peer_map.get(&token).unwrap()
    }

    fn peer_mut_ref(&mut self, token: Token) -> &mut PeerSocket {
        self.peer_map.get_mut(&token).unwrap()
    }

    fn addr_is_valid(&self, peer_addr: &SocketAddr) -> bool {
        !self.existed_in_waiting_list(&peer_addr) && !self.existed_in_peer_map(&peer_addr) && !self.existed_in_ban_list(&peer_addr)
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
                            if self.addr_is_valid(&peer_addr) {
                                new_peers.push((peer.addr(), peer));
                            }
                        },
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            // try again
                            println!("Server socket is not ready anymore, stop accepting");
                        },
                        e => {
                            panic!("{:?}", e)
                        }
                    }
                },
                peer_token => {
                    // process peer event
                    //let peer = self.peer_mut_ref(peer_token);
                    let result = self.peer_mut_ref(peer_token).store_buffer();
                    self.eventloop.reregister_peer(peer_token.clone(), self.peer_ref(peer_token));
                }
            }
        }
        // put new peers in the waiting list
        self.append_waiting_list(&mut new_peers);
    }

    // remove all dead peer
    fn remove_dead_peers(&mut self) {
        let dead_tokens: Vec<Token> = self.peer_map.iter().filter(|&(_token, peer)| {
            peer.is_alive()
        }).map(|(token, _peer)| {
            token.clone()
        }).collect();

        for token in dead_tokens {
            self.peer_map.remove(&token);
        }
    }

    // select all prepared socket and send data
    fn send_all(&mut self) {
        self.peer_map.iter_mut().filter(|&(ref _token, ref peer)| {
            peer.prepare_to_send_data()
        }).for_each(|(_token, peer)| {
            peer.send_buffer().unwrap();
        });
    }

    // select all prepared socket and read message
    fn read_all(&mut self) {
        let mut dispatch_msgs: Vec<SocketMessage> = vec![];
        self.peer_map.iter_mut().filter(|&(ref _token, ref peer)| {
            peer.prepare_to_recv_msg()
        }).for_each(|(_token, peer)| {
            if let Ok(msgs) = peer.read_msg() {
                for msg in msgs {
                    dispatch_msgs.push(msg);
                }
            }
        });
        for msg in dispatch_msgs {
            self.msg_listener.lock_trait_obj().notify(msg);
        }
    }
}

impl Processor for P2PManager {
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
        // do nothing
    }

    fn exec(&mut self) -> bool {
        match self.eventloop.next_tick() {
            Ok(_size) => {
                self.process_events();
                self.remove_dead_peers();
                self.send_all();
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

    fn time_span(&self) -> u64 { TIME_SPAN }
}

impl Drop for P2PManager {
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