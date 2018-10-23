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

const ROUND_HEART_BEAT: usize = 50;
const ROUND_PRUNE: usize = 50;

/// Socket message listener
pub trait SocketMessageListener: Send {
    fn notify(&mut self, msg: SocketMessage);
}

pub struct P2PConfig {
    port: u16,
    peer_map_limit: usize,
    ban_list_limit: usize,
    waiting_list_limit: usize,
    min_peers: usize,
    white_list: Vec<SocketAddr>,
    bootstrap_peers: Vec<SocketAddr>,
}

impl P2PConfig {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn peer_map_limit(&self) -> usize {
        self.peer_map_limit
    }

    pub fn ban_list_limit(&self) -> usize {
        self.ban_list_limit
    }

    pub fn waiting_list_limit(&self) -> usize {
        self.waiting_list_limit
    }

    pub fn min_peers(&self) -> usize {
        self.min_peers
    }

    pub fn white_list(&self) -> Vec<SocketAddr> {
        self.white_list.clone()
    }

    pub fn set_white_list(&mut self, list: Vec<SocketAddr>) {
        self.white_list = list
    }

    pub fn in_white_list(&self, new_addr: &SocketAddr) -> bool {
        if self.white_list.is_empty() {
            true
        } else {
            self.white_list.iter().any(|addr| {
                new_addr == addr
            })
        }
    }

    pub fn bootstrap_peers(&self) -> Vec<SocketAddr> {
        self.bootstrap_peers.clone()
    }
}

/// P2PController
pub struct P2PManager {
    name: String,
    thread_status: ThreadStatus,
    listener: TcpListener,
    receiver: Option<Receiver<Message>>,
    peer_map: HashMap<Token, PeerSocket>,
    ban_list: Vec<SocketAddr>,
    waiting_list: Vec<SocketAddr>,
    eventloop: NetworkEventLoop<PeerSocket>,
    msg_listener: ContextRef<SocketMessageListener>,
    config: P2PConfig
}

impl P2PManager {
    fn new(
        name: String,
        msg_listener: ContextRef<SocketMessageListener>,
        event_size: usize,
        config: P2PConfig
    ) -> Result<Self> {
        let server_addr: SocketAddr = format!("127.0.0.1:{}", config.port()).parse().unwrap();
        match TcpListener::bind(&server_addr) {
            Ok(listerner) => Ok(P2PManager {
                name: name,
                thread_status: ThreadStatus::Pause,
                listener: listerner,
                receiver: None,
                peer_map: HashMap::new(),
                ban_list: Vec::new(),
                waiting_list: Vec::new(),
                eventloop: NetworkEventLoop::new(event_size),
                msg_listener: msg_listener,
                config: config
            }),
            Err(e) => Err(e)
        }
    }

    /// Create a P2PController.
    /// Return contextref.
    pub fn create(
        name: String,
        config: P2PConfig,
        event_size: usize,
        stack_size: usize,
        msg_listener: ContextRef<SocketMessageListener>
    ) -> Result<ContextRef<Self>> {
        P2PManager::new(
            name,
            msg_listener,
            event_size,
            config
        ).and_then(|controller| {
            Ok(controller.launch(stack_size))
        })
    }

    pub fn set_msg_listener(&mut self, listener: ContextRef<SocketMessageListener>) {
        self.msg_listener = listener;
    }

    // Obtain a new peer from the waiting list
    fn obtain_peer(&mut self) -> Result<Token> {
        if let Some(addr) = self.waiting_list.pop() {
            if !self.addr_is_valid_to_accept(&addr) {
                return Err(Error::new(ErrorKind::Other, "The address is not valid"));
            }
            match TcpStream::connect(&addr) {
                Ok(stream) => {
                    let mut peer = PeerSocket::new(stream);
                    // register peer
                    self.accept_peer(peer)
                },
                Err(e) => Err(e)
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Waiting list is empty"))
        }
    }

    /// Accept and register peer
    fn accept_peer(&mut self, mut peer: PeerSocket) -> Result<Token> {
        if self.peer_map.len() >= self.config.peer_map_limit() {
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
    fn drop_peer(&mut self, token: Token) -> Result<()> {
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
    pub fn append_waiting_list(&mut self, new_peers: &mut Vec<SocketAddr>) {
        let remain_size = self.config.waiting_list_limit() - self.waiting_list.len();
        let end_pos = if remain_size > self.waiting_list.len() {
            self.waiting_list.len()
        } else {
            remain_size
        };
        let mut target_peers: Vec<SocketAddr> = new_peers.drain(0..end_pos).collect();
        self.waiting_list.append(&mut target_peers);
    }

    /// Drop all peers in the waiting list
    fn clear_waiting_list(&mut self) {
        self.waiting_list = Vec::new();
    }

    /// Existed in waiting list or not
    fn existed_in_waiting_list(&self, addr: &SocketAddr) -> bool {
        self.waiting_list.iter().any(|elem| {
            elem == addr
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

    fn peer_info(&self) -> Vec<SocketAddr> {
        let peer_info: Vec<SocketAddr> = self.peer_map.iter().map(|(_token, peer)| {
            peer.addr()
        }).collect();
        peer_info
    }

    fn addr_is_valid_to_accept(&self, peer_addr: &SocketAddr) -> bool {
        !self.existed_in_peer_map(&peer_addr) && !self.existed_in_ban_list(&peer_addr)
    }

    fn addr_is_valid_for_waiting_list(&self, peer_addr: &SocketAddr) -> bool {
        !self.existed_in_waiting_list(peer_addr)
            && !self.existed_in_ban_list(peer_addr)
            && !self.existed_in_peer_map(peer_addr)
            && self.config.in_white_list(peer_addr)
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
                            // accept peer if valid
                            if self.addr_is_valid_to_accept(&peer_addr) {
                               match self.accept_peer(peer) {
                                   Ok(r) => info!("New peer({:?}) accepted", r),
                                   Err(e) => warn!("Failed to accept new peer, {:?}", e)
                               }
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
    }

    // remove all dead peer
    fn remove_dead_peers(&mut self) {
        if self.eventloop.round > 0 &&  self.eventloop.round % ROUND_PRUNE == 0 {
            let dead_tokens: Vec<Token> = self.peer_map.iter().filter(|&(_token, peer)| {
                peer.is_alive()
            }).map(|(token, _peer)| {
                token.clone()
            }).collect();

            for token in dead_tokens {
                self.peer_map.remove(&token);
            }
        }
    }

    // select all prepared socket and send data
    fn send_all(&mut self) {
        self.peer_map.iter_mut().filter(|&(ref _token, ref peer)| {
            peer.prepare_to_send_data()
        }).for_each(|(token, peer)| {
            match peer.send_buffer() {
                Ok(r) => {},
                Err(e) => warn!("Can not send data to peer({:?}), {:?}", token, e)
            }
        });
    }

    // select all prepared socket and read message
    fn read_all(&mut self) {
        let mut new_waiting_peers: Vec<SocketAddr> = vec![];
        let mut dispatch_msgs: Vec<SocketMessage> = vec![];
        let peer_info = self.peer_info();
        self.peer_map.iter_mut().filter(|&(ref _token, ref peer)| {
            peer.prepare_to_recv_msg()
        }).for_each(|(_token, peer)| {
            if let Ok(msgs) = peer.read_msg() {
                for msg in msgs {
                    if msg.is_heartbeat() {
                        continue
                    }
                    if msg.is_discovery() {
                        SocketMessage::peer_info(peer_info.clone());
                        continue
                    }
                    if msg.is_exception() {
                        if let Some(reason) = msg.exception_msg() {
                            warn!("Peer receive an exception: {:?}, try to close connection...", reason);
                            peer.kill();
                        }
                        continue
                    }
                    if msg.is_peer_info() {
                        match msg.parse_peer_info() {
                            Ok(mut new_list) => {
                                new_waiting_peers.append(&mut new_list);
                            },
                            _ => {}
                        };
                        continue
                    }
                    dispatch_msgs.push(msg);
                }
            }
        });
        new_waiting_peers.dedup();
        new_waiting_peers = new_waiting_peers.iter().filter(|&addr| {
            self.addr_is_valid_for_waiting_list(addr)
        }).map(|addr| {
            addr.clone()
        }).collect::<Vec<SocketAddr>>();
        self.append_waiting_list(&mut new_waiting_peers);
        for msg in dispatch_msgs {
            self.msg_listener.lock_trait_obj().notify(msg);
        }
    }

    // send heart beat to all peers
    #[inline]
    fn send_heart_beat(&mut self) {
        let tokens: Vec<Token> = self.peer_map.iter_mut().map(|(token, peer)| {
            peer.update_ttl();
            token.clone()
        }).collect();
        for token in tokens.clone() {
            self.send_msg(token, SocketMessage::heartbeat());
        }
        if self.eventloop.round > 0 && self.eventloop.round % ROUND_HEART_BEAT == 0 {
            for token in tokens {
                self.send_msg(token, SocketMessage::heartbeat());
            }
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
                self.send_heart_beat();
                self.remove_dead_peers();
                self.send_all();
                // obtain a new peer
                if self.peer_map.len() < self.config.min_peers {
                    match self.obtain_peer() {
                        Ok(token) => info!("Discover a new peer({:?})", token),
                        Err(e) => warn!("Can not descover a new peer: {:?}", e)
                    }
                }
            },
            Err(e) => {
                panic!("exception: {:?}", e);
            }
        }
        true
    }

    fn pre_exec(&mut self) -> bool {
        self.waiting_list = self.config.bootstrap_peers();
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