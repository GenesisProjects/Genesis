extern crate common;

use self::common::hash::*;

/*type (
fullNode struct {
    Children [17]node // Actual trie node data to encode/decode (needs custom encoder)
    flags    nodeFlag
}
shortNode struct {
    Key   []byte
    Val   node
    flags nodeFlag
}
hashNode  []byte
valueNode []byte
)
type node interface {
	fstring(string) string
	cache() (hashNode, bool)
	canUnload(cachegen, cachelimit uint16) bool
}

*/

/*
type nodeFlag struct {
    hash  hashNode // cached hash of the node (may be nil)
    gen   uint16   // cache generation counter
    dirty bool     // whether the node has changes that must be written to the database
}
*/

pub type TrieKey = Hash;

pub struct ExtensionNode {
    shared_encoded_path: Vec<u8>,
    key: TrieKey
}

pub struct ValueNode<T> {
    shared_encoded_path: Vec<u8>,
    value: T
}

pub struct BranchNode<T> {
    router: Vec<TrieKey>,
    value: T
}

/*pub struct NodeFlag {
    hash: Hash,
    gen: u16,
    dirty: bool
}*/

pub trait NodeOp {

}
