use rand::{Rng};
use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
};
use rug::{Integer, Assign};
use rug::ops::{Pow, BitXorFrom};

type NodeID = Integer;

pub struct Value {
    data: Vec<u8>,
}

impl Value {
    pub fn new() -> Value {
        let zero_vec = vec![0u8; 10];
        Value {
            data: zero_vec.clone(),
        }
    }
}

// Kademlia effectively treats nodes as leaves in a binary tree, 
// with each node's position determined by the shortest unique prefix
// of its ID. 
pub struct Node {
    id: NodeID,
    value: Value,
    // key: usize,   // 160-bit SHA1 hash of value
    // buckets: Vec<Bucket>,
    // left_child: &Node,
    // right_child: &Node,
}

// Assign 160-bit opaque IDs to nodes
const NODE_ID_LENGTH: usize = 160;
const K: usize = 20;


// A node organizes its contacts, other nodes known to it, in buckets
// which hold a maximum of k contacts. These are known as k-buckets. 
// The buckets are organized by the distance b/w the node and the 
// contacts in the bucket. Specifically, for bucket j, where 0 <= j < k, 
// we are guaranteed that 
// 2^j <= distance(node, contact) < 2^(j+1)
pub struct Bucket {
    contacts: [Contact; K],
}

pub struct Contact {
    socket: SocketAddr,
    id: NodeID,
}

impl Bucket {
    // Within buckets, contacts are sorted by the time of the most recent 
    // communication, with those which have most recently communicated at the
    // end of the list and those which have least recently communicated at the front,
    // regardless of whether the node or the contact initiated the sequence of msgs
    fn sort() {

    }

    // Whenever a node receives a communication from another, it updates the 
    // corresponding bucket. If the contact already exists, it is moved to the end 
    // of the bucket. Otherwise, if the bucket is not full, the new contact is added
    // at the end. 
    // fn update(&self, contact: Contact) {
    //     if self.contains(contact) {
    //         // move contact to the end of the bucket
    //     } else {
    //         if self.length() != K {
    //             // if bucket isn't full, add to end
    //             self.append(contact);
    //         } else {
    //             // node pings the contact at the head of the bucket's list
    //             // if that least recently seen contact fails to respond in 
    //             // an (unspecified) reasonable time, it is dropped from the list,
    //             // and the new contact is added at the tail. Otherwise the 
    //             // new contact is ignored for bucket updating purposes.
    //         }
    //     }
    // }
}


impl Node {

    pub fn new() -> Node {

        // Generate the key, then assign it to an ID
        // let mut gen = OsRng::new().expect("Failed to get OS random generator");
        // let mut random: Vec<u8> = repeat(0u8).take(20).collect();
        // gen.fill_bytes(&mut key[..]);
        // let mut rng = rand::thread_rng();
        let base = Integer::from(2);
        let power = base.pow(160);
        // let gen_id = rng.gen_range(0, base.pow(60));
        Node {
            id: power,
            value: Value::new(),

        }
    }

    // Provide a lookup algorithm that locates succesively "closer" nodes 
    // to any desired ID, converging to the lookup target in logarithmically
    // many steps.
    pub fn lookup(&self, target: Node) {

    }
}

// impl NodeID {
//     // Given two 160-bit identifiers, key1 and key2, Kademlia defines the 
//     // distance between them as their bitwise exclusive (or XOR) interpreted
//     // as an integer
//     pub fn distance(key1: NodeID, key2: NodeID) -> usize {
//         key1 ^ key2
//     }
// }