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

use super::node::types::*;
use super::node::op::*;
use self::db::manager::*;

struct Trie<T : NodeOp> {
    db: &'static DBManager,
    root: T,
    cachegen: u16,
    cachelimit: u16
}

impl<T: NodeOp> Trie<T> {
    /// set_cache_limit sets the number of 'cache generations' to keep.
    /// A cache generation is created by a call to Commit.
    pub fn set_cache_limit(&mut self, limit: u16) {
        self.cachelimit = limit;
    }

    /// Set_new_flag returns the cache flag value for a newly created node.
    pub fn set_new_flag(&self) -> NodeFlag {
        NodeFlag{ dirty: true, gen: self.cachegen, hash: None }
    }

    /// New creates a trie with an existing root node from db.
    ///
    /// If root is the zero hash or the sha3 hash of an empty string, the
    /// trie is initially empty and does not require a database. Otherwise,
    /// New will panic if db is nil and returns a MissingNodeError if root does
    /// not exist in the database. Accessing the trie loads nodes from db on demand.
    fn new_from_db(root: T, db: &'static DBManager){

    }
}


