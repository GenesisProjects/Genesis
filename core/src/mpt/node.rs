extern crate common;

use std::iter::Iterator;
use self::common::hash::*;
use std::fmt::{ Debug, Formatter };

static INDICES: [&'static str; 17] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "[17]"];

pub type TrieKey = Hash;

pub type ExtensionNode = Vec<u8>;

pub type ValueNode = Vec<u8>;

pub struct BranchNode<'a> {
    router: [Option<&'a NodeOp>; 17],
    flags: NodeFlag<'a>
}

#[derive(Default)]
pub struct NodeFlag<'a> {
    /// cached hash of the node (may be nil)
    pub hash: Option<&'a ExtensionNode>,
    /// cache generation counter
    pub gen: u16,
    /// whether the node has changes that must be written to the database
    pub dirty: bool

}

impl<'a> NodeFlag<'a> {
    pub fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool {
        !self.dirty && cachegen - self.gen >= cachelimit
    }
}

pub trait NodeOp {
    fn print(&self, index: &String) -> String;

    fn cache(&self) -> (Option<& ExtensionNode>, bool);

    fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool;
}

impl<'a> NodeOp for BranchNode<'a> {
    fn print(&self, index: &String) -> String {
        let mut buff = "".to_string();
        buff = buff + format!("[\n{}\t", index.as_str()).as_str();
        for (i, elem) in self.router.iter().enumerate() {
            match *elem {
                Some(n) => buff = buff + format!("{}: {}", INDICES[i], (*n).print(&(index.clone() + "\t")).as_str()).as_str(),
                None => buff = buff + format!("{}: <nil> ", INDICES[i]).as_str(),
            };
        }
        buff + format!("\n{}] ", index).as_str()
    }

    fn cache(&self) -> (Option<& ExtensionNode>, bool) {
        (self.flags.hash, self.flags.dirty)
    }

    fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool {
       self.flags.can_unload(cachegen, cachelimit)
    }
}