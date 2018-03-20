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

type HashNode = Vec<u8>;
type ValueNode = Vec<u8>;


pub struct NodeFlag {
    hash: HashNode,
    gen: u16,
    dirty: bool
}

pub trait NodeOp {

}

pub struct ShortNode {
    children: [NodeOp; 17] // Actual trie node data to encode/decode (needs custom encoder)
}

