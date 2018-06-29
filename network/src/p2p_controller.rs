use peer::*;
use session::*;
use utils::*;

use std::collections::HashMap;
use std::io::*;
use std::sync::{Mutex, Arc, Condvar};
use std::time::Duration;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use common::address::Address as Account;
use common::gen_message::*;
use common::thread::{Thread, ThreadStatus};
use common::observe::Observe;

use nat::*;

use mio::*;
use mio::net::{TcpListener, TcpStream};

const SERVER_TOKEN: Token = Token(0);

lazy_static! {
    pub static ref TOKEN_SEQ: Mutex<usize> = {
        Mutex::new(1usize)
    };
}

fn token_generator() -> Token {
    let mut seq = TOKEN_SEQ.lock().unwrap();
    let token = Token(*seq);
    *seq += 1;
    token
}

struct NetworkEventLoop {
    loop_count: usize,
    events: Events,
    poll: Poll,
    status: ThreadStatus
}

impl NetworkEventLoop {
    pub fn new(events_size: usize) -> Self {
        // Event storage
        let mut events = Events::with_capacity(events_size);
        // The `Poll` instance
        let poll = Poll::new().expect("Can not instantialize poll");

        NetworkEventLoop {
            loop_count: 0usize,
            events: events,
            poll: poll,
            status: ThreadStatus::Stop
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

    fn next_tick(&mut self) -> Result<usize> {
        //TODO: make loop span configurable
        self.poll.poll(&mut self.events, Some(Duration::from_millis(10))).and_then(|events_size| {
            self.loop_count += 1;
            Ok(events_size)
        })
    }
}


///
///
pub struct P2PController {
    account: Account,
    peer_list: HashMap<Token, PeerRef>,
    max_allowed_peers: usize,
    waitting_list: Vec<SocketAddr>,
    max_waiting_list: usize,
    block_list: Vec<SocketAddr>,
    max_blocked_peers: usize,
    eventloop: NetworkEventLoop,
    listener: TcpListener,
    ch_pair: Option<Arc<(Mutex<MessageChannel>, Condvar)>>
}

impl P2PController {
    pub fn launch_controller(name: String) {
        P2PController::launch::<P2PController>(name);
    }

    pub fn bootstrap(&mut self) {
        //TODO: port configuable
        let socket_info = match get_local_ip() {
            Some(socket_info) => get_public_ip_addr(Protocol::UPNP, &(SocketAddr::new(socket_info, 19999), 19999)),
            None => None
        };

        self.init_peers_table();

        for token in self.peer_list.keys() {
            let peer_ref = self.peer_list.get(token);
            let addr = peer_ref.unwrap().clone().addr();
            let _ = TcpStream::connect(&addr);
        }

        unimplemented!()
    }

    fn init_peers_table(&mut self) {
        unimplemented!()
    }

    fn search_peers(&self) -> Vec<(Account, SocketInfo)> {
        let mut raw_peers_table = self.peer_list.values().map(|peer_ref| {
            peer_ref.peer_table()
        }).fold(Vec::<(Account, SocketInfo)>::new(), |mut init, ref mut table: Vec<(Account,SocketInfo)>| {
            init.append(table);
            init
        });

        // filter out identical elements
        raw_peers_table.sort_by(|&(ref addr_a, _), &(ref addr_b, _)| addr_a.partial_cmp(addr_b).unwrap());
        raw_peers_table.dedup_by(|&mut (ref addr_a, _), &mut (ref addr_b, _)| *addr_a == *addr_b);

        // filter out self
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref addr, _)| *addr != self.account).collect();

        // filter out in current peer list
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref account, (ref addr, ref port))| !self.socket_exist(addr)).collect();

        // filter out in block list
        raw_peers_table = raw_peers_table.into_iter().filter(|&(ref account, (ref addr, ref port))| !self.socket_blocked(addr)).collect();

        raw_peers_table
    }

    fn socket_exist(&self, addr: &SocketAddr) -> bool {
        match self.peer_list.iter().find(|&(token, peer_ref)| {
            peer_ref.addr() == *addr
        }) {
            Some(_) => true,
            _ => false
        }
    }

    fn socket_blocked(&self, addr: &SocketAddr) -> bool {
        match self.block_list.iter().find(|&blocked_addr| {
            *blocked_addr == *addr
        }) {
            Some(_) => true,
            _ => false
        }
    }

    fn peers_persist(&self) -> Result<usize> {
        unimplemented!()
    }

    fn reflesh_peer_list(&mut self) {
        unimplemented!()
    }

    fn add_peer(peer_ref: PeerRef) {
        unimplemented!()
    }

    fn remove_peer(peer_ref: PeerRef) {
        unimplemented!()
    }

    fn ban_peer(addr: Account, loops: usize) {
        unimplemented!()
    }

    fn register<'a>(&self, peer_ref: PeerRef, local_block_info: &'a BlockInfo) -> Result<BlockInfo> {
        unimplemented!()
    }

    fn process_events(&mut self) {
        for event in &(self.eventloop.events) {
            match event.token() {
                SERVER_TOKEN => {
                    match self.listener.accept() {
                        Ok((socket, _)) => {

                        },
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            // EAGAIN
                        },
                        e => {

                        }
                    }
                },
                PEER_TOKEN => {

                }
            }
        }
    }

}

impl Observe for P2PController {
    fn subscribe(&mut self, name: String) {
        self.ch_pair = Some(
            MESSAGE_CENTER
            .lock()
            .unwrap()
            .subscribe(&name)
            .clone()
        );
    }

    fn unsubscribe(&mut self) {
        if let Some(ch_pair) = self.ch_pair.clone() {
            let uid = (*ch_pair).0.lock().unwrap().uid.clone();
            self.ch_pair = None;
            MESSAGE_CENTER
                .lock()
                .unwrap()
                .unsubscribe(&"P2P_CONTROLLER".to_string(), uid);
        }

    }

    fn receive_async(&mut self) -> Option<Message> {
        if let Some(ch_pair) = self.ch_pair.clone() {
            (*ch_pair).0.lock().unwrap().accept_msg_async()
        } else {
            None
        }
    }

    fn receive_sync(&mut self) -> Message {
        if let Some(ch_pair) = self.ch_pair.clone() {
            let condvar_ref = &((*ch_pair).1);
            let lock_ref = &((*ch_pair).0);
            if let Some(msg) = lock_ref.lock().unwrap().accept_msg_async().clone() {
                msg
            } else {
                loop {
                    let msg = condvar_ref
                        .wait(lock_ref.lock().unwrap())
                        .unwrap()
                        .accept_msg_async();

                    match msg {
                        Some(msg) => { return msg; }
                        None => { continue; }
                    }
                }
            }
        } else {
            panic!("No channel subscribed")
        }
    }
}

impl Thread for P2PController {
    fn new() -> Result<Self> {
        //TODO: load port from config
        let addr = "127.0.0.1:39999".parse().unwrap();
        let server = TcpListener::bind(&addr);
        let account = Account::load();

        match (server, account) {
            (Ok(server), Some(account)) => {
                //TODO: load events size from config
                let event_loop = NetworkEventLoop::new(1024);
                //TODO: max_allowed_peers configuable
                let max_allowed_peers = 512;
                //TODO: max_blocked_peers configuable
                let max_blocked_peers = 1024;
                //TODO: max_waiting_list configuable
                let max_waiting_list = 1024;

                let mut peer_list = HashMap::<Token, PeerRef>::new();
                Ok(P2PController {
                    account: account.clone(),
                    peer_list: peer_list,
                    max_allowed_peers: max_allowed_peers,
                    waitting_list: vec![],
                    max_waiting_list: max_waiting_list,
                    block_list: vec![],
                    max_blocked_peers: max_blocked_peers,
                    eventloop: event_loop,
                    listener: server,
                    ch_pair: None
                })
            },
            (Ok(_), None) => {
                Err(Error::from(ErrorKind::ConnectionRefused))
            },
            (Err(e), _) => {
                Err(e)
            }
        }
    }

    fn run(&mut self) -> bool {
        // fetch the next tick
        let result = self.eventloop.next_tick();
        match self.eventloop.status {
            ThreadStatus::Running => {
                match result {
                    Ok(size) => {
                        println!("{} events are ready", size);
                        self.process_events();
                        true
                    },
                    Err(_) => false
                }
            },
            ThreadStatus::Stop => false,
            ThreadStatus::Pause => true
        }
    }

    /// update with new msg
    fn update(&mut self, msg: Message) {
        unimplemented!()
    }
    fn set_status(&mut self, status: ThreadStatus) {
        self.eventloop.status = status;
    }
}