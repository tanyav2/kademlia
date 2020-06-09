extern crate node;
extern crate routing;

use crypto::sha1::Sha1;

// Use gRPC

// In all RPCs, the recipient much echo a 160-bit random RPC ID, which 
// provides some ressitance to address forgery. 
//
// PINGs can also be piggy
// backed on RPC replies for the RPC recipient to obtain additional assurance
// of the sender's network address (??? TODO)

// every msg a node transmits includes its node ID, permitting the recipient
// to record the sender's existence if necessary
struct RPCRequest<T> {
    request: T,
    send_id: NodeID,
}

struct RPCResponse<T> {
    response: T, 
    recv_id: NodeID, 
    error: RPCError,
}

// Probes a node to see if it is online
fn ping() -> RPCResponse<bool> {

}

// Store instructs a node to store a <key, value> pair for later retrieval
fn store(data: String) -> RPCResponse<None> {
    // generate key as a hash of the value
    let mut hasher = Sha1::new();
    hasher.input_str(data);
    let key = hasher.result_str();
    
}

// Takes a 160-bit ID as an argument. 
// The recipient of the RPC returns <IP address, UDP port, Node ID> triples
// for the k nodes it knows about closest to the target ID. 
// These triples can come from a single k-bucket, or they may come from 
// multiple k-buckets if the closest k-bucket is not full. In any case, 
// the RPC recipient must return k items (unless there are fewer than k nodes
// in all its k-buckets combined, in which case it returns every node it knows
// about)
fn find_node(id: NodeID) -> RPCResponse<Vec<NodeFound>> {


}

enum NodeValue {
    NodeFound,
    NodeVal,
}

// find_value behaves like find_node -- returning <Ip addr, UDP port, Node ID>
// triples -- with one exception. If the RPC recipient has received a Store RPC
// for the key, it just returns the stored value.
fn find_value(id: NodeID) -> RPCResponse<NodeValue> {

}