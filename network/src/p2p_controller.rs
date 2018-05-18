use peer::*;
use session::*;

use std::collections::HashMap;

use common::address::Address;

pub struct P2PController {
    session_list: HashMap<String, Session>,
    black_list: HashMap<String, Address>,
}

impl P2PController {
    
}