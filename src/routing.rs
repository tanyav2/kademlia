extern crate node;

// Kademlia's basic routing table structure is fairly straight-forward
// given the protocol, though a slight subtlety is needed to handle highly 
// unbalanced trees. 

// The routing table is a binary tree whose leaves are k-buckets.

// alpha is a small number representing the degree of parallelism in network calls
const ALPHA: usize = 3;

// Maximum number of contacts stored in a bucket

pub struct BinTree {
    root: Node, 
    
}


// For any given node, we divide the binary tree into a series of successively 
// lower subtrees that don't contain the node. The highest subtree consists of
// the half of the binary tree not containing the node
impl BinTree {
    fn insert(&self, node: Node) {

    }

    fn delete(&self, node: Node) {

    }
}
