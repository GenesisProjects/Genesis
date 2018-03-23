extern crate common;
extern crate rlp;

use std::iter::Iterator;
use std::fmt::{ Debug, Formatter, Result};
use self::common::hash::*;
use self::rlp::RLPSerialize;

static INDICES: [&'static str; 17] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "[17]"];

pub mod op {
    use super::types::*;

    pub trait NodeOp {
        fn print(&self, index: &String) -> String;
        fn cache(&self) -> (Option<HashNode>, bool);
        fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool;
    }
}

pub mod types {
    use super::op::*;

    #[derive(Clone, Debug)]
    pub struct HashNode { pub hash: Vec<u8> }

    #[derive(Clone, Debug)]
    pub struct ValueNode { pub value: Vec<u8> }

    pub struct BranchNode<'a> {
        pub router: [Option<&'a NodeOp>; 17],
        pub flags: NodeFlag
    }

    pub struct ShortNode<'a> {
        pub key: Vec<u8>,
        pub value: &'a NodeOp,
        pub flags: NodeFlag
    }

    #[derive(Default)]
    pub struct NodeFlag {
        /// cached hash of the node (may be nil)
        pub hash: Option<HashNode>,
        /// cache generation counter
        pub gen: u16,
        /// whether the node has changes that must be written to the database
        pub dirty: bool
    }
}

mod node_impl {
    use super::types::*;
    use super::op::*;

    impl NodeFlag {
        pub fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool {
            !self.dirty && cachegen - self.gen >= cachelimit
        }
    }

    impl<'a> NodeOp for BranchNode<'a> {
        fn print(&self, index: &String) -> String {
            let mut buff = "".to_string();
            buff = buff + format!("[\n{}\t", index.as_str()).as_str();
            for (i, elem) in self.router.iter().enumerate() {
                match *elem {
                    Some(n) => buff = buff + format!("{}: {}", super::INDICES[i], (*n).print(&(index.clone() + "\t")).as_str()).as_str(),
                    None => buff = buff + format!("{}: <nil>\t", super::INDICES[i]).as_str(),
                };
            }
            buff + format!("\n{}] ", index).as_str()
        }

        fn cache(&self) -> (Option<HashNode>, bool) {
            (self.flags.hash.clone(), self.flags.dirty)
        }

        fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool {
            self.flags.can_unload(cachegen, cachelimit)
        }
    }

    impl<'a> NodeOp for ShortNode<'a> {
        fn print(&self, index: &String) -> String {
            format!("{}: {}\t", String::from_utf8(self.key.clone()).unwrap().as_str(), self.value.print(&(index.clone() + "\t")).as_str())
        }

        fn cache(&self) -> (Option<HashNode>, bool) {
            (self.flags.hash.clone(), self.flags.dirty)
        }

        fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool {
            self.flags.can_unload(cachegen, cachelimit)
        }
    }

    impl<'a> NodeOp for HashNode {
        fn print(&self, index: &String) -> String {
            format!("<{}> ", String::from_utf8(self.hash.clone()).unwrap())
        }

        fn cache(&self) -> (Option<HashNode>, bool) { (None, true) }

        fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool { false }
    }

    impl<'a> NodeOp for ValueNode {
        fn print(&self, index: &String) -> String {
            format!("<{}> ", String::from_utf8(self.value.clone()).unwrap())
        }

        fn cache(&self) -> (Option<HashNode>, bool) { (None, true) }

        fn can_unload(&self, cachegen: u16, cachelimit: u16) -> bool { false }
    }
}

mod rlp_imp {
    use super::types::*;
    use super::op::*;
    use super::rlp::types::{ RLP, RLPError };

    impl<'a> super::RLPSerialize for BranchNode<'a> {
        fn encode(&self) -> Result<RLP, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }

        fn decode(rlp: &RLP) -> Result<Self, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }
    }

    impl<'a> super::RLPSerialize for ShortNode<'a> {
        fn encode(&self) -> Result<RLP, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }

        fn decode(rlp: &RLP) -> Result<Self, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }
    }

    impl<'a> super::RLPSerialize for HashNode {
        fn encode(&self) -> Result<RLP, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }

        fn decode(rlp: &RLP) -> Result<Self, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }
    }

    impl<'a> super::RLPSerialize for ValueNode {
        fn encode(&self) -> Result<RLP, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }

        fn decode(rlp: &RLP) -> Result<Self, RLPError> {
            Err(RLPError::RLPErrorUnknown)
        }
    }
}