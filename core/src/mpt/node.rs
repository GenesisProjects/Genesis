extern crate common;

use std::iter::Iterator;
use self::common::hash::*;
use std::fmt::{ Debug, Formatter };

static INDICES: [String; 17] = ["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "[17]".to_string()];

pub type TrieKey = Hash;

pub type ExtensionNode = Vec<u8>;

pub type ValueNode = Vec<u8>;

pub struct BranchNode<'a> {
    router: [Option<&'a NodeOp>; 17],
    flags: NodeFlag
}

#[derive(Default)]
pub struct NodeFlag {
    /// cached hash of the node (may be nil)
    pub hash: Option<ExtensionNode>,
    /// cache generation counter
    pub gen: u16,
    /// whether the node has changes that must be written to the database
    pub dirty: bool

}

impl NodeFlag {
    pub fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool {
        !self.dirty && cachegen - self.gen >= cachelimit
    }
}

pub trait NodeOp {
    fn print(&self, index: &String) -> String;

    fn cache(&self) -> (Option<ExtensionNode>, bool);

    fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool;
}

impl<'a> NodeOp for BranchNode<'a> {
    fn print(&self, index: &String) -> String {
        let mut buff = "".to_string();
        buff = buff + format!("[\n{}\t", index.as_str()).as_str();
        for (i, elem) in self.router.iter().enumerate() {
            match *elem {
                Some(n) => buff = buff + format!("{}: {}", INDICES[i].as_str(), (*n).print(&(*index + "\t")).as_str()).as_str(),
                None => buff = buff + format!("{}: <nil> ", INDICES[i].as_str()).as_str(),
            };
        }
        buff + format!("\n{}] ", index).as_str()
    }

    fn cache(&self) -> (Option<ExtensionNode>, bool) {
        (self.flags.hash, self.flags.dirty)
    }

    fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool {
       self.flags.can_unload(cachegen, cachelimit)
    }
}