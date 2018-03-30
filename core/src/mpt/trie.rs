/*
type Trie struct {
    db           *Database
    root         node
    originalRoot common.Hash

    // Cache generation values.
    // cachegen increases by one with each commit operation.
    // new nodes are tagged with the current generation and unloaded
    // when their generation is older than than cachegen-cachelimit.
    cachegen, cachelimit uint16
}*/
extern crate common;
extern crate db;

use super::node::*;
use self::db::manager::*;

struct Trie {

}

impl Trie {

}


