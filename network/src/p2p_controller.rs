use peer::*;

use std::collections::HashMap;

use common::address::Address;

struct PeerInfo {
    peer: Peer,
}

pub struct P2PController {
    peers_list: HashMap<String, PeerInfo>,
    black_list: HashMap<String, PeerInfo>,
}