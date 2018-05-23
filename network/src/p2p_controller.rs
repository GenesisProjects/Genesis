use peer::*;
use session::*;

use std::collections::HashMap;
use std::io::*;

use mio::*;
use mio::net::{TcpListener, TcpStream};

use common::address::Address;

pub struct P2PController {
    loop_count: usize,
    peer_list: Vec<(PeerRef, usize)>,
    black_list: Vec<(Address, usize)>,

    listener: Option<TcpListener>,
    token_map: HashMap<Token, PeerRef>
}

impl P2PController {
    fn search_peers(&self) -> Vec<PeerRef> {
        unimplemented!()
    }

    fn default_peers(&self) -> Vec<PeerRef> {
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

    fn ban_peer(addr: Address, loops: usize) {
        unimplemented!()
    }

    fn connect_peer<'a>(peer_ref: PeerRef, local_block_info: &'a BlockInfo) -> Result<BlockInfo> {
        unimplemented!()
    }

    fn new() -> Self {
        unimplemented!()
    }

    fn init(&mut self) {
        unimplemented!()
    }

    fn start_run_loop(&self) {
        loop {
            unimplemented!()
        }
    }
}