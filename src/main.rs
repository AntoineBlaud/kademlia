
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, RwLock, PoisonError};
use rand::random;


mod node;
mod request;

use crate::node::{Node, NodeInfo, helper_start_node};


// I need to unsterstand how kamdemlia works when the network is small but the id space is large.
// Because creating kbucket with node from certain branch of the tree is not possible when the network is small.



fn main() {
    let mut nodes = Vec::new();

    // Create 10 nodes
    for i in 0..10 {
        let id = rand::random::<u64>();
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = (rand::random::<u16>()% 50000 + 1000); 
        let nd_info = NodeInfo {id, ip_addr, port, last_seen : std::time::Instant::now()};
        nodes.push(Arc::new(RwLock::new(Node::new(nd_info))));
    }

    // Start all listening nodes threads
    for i in 0..10 {
        helper_start_node(nodes[i].clone());
        std::thread::sleep(std::time::Duration::from_millis(200));
   }
    
    // Fetch node_info from node 0, we will use this node as a bootstrap node
    let root_node = &nodes[0];
    let guard = root_node.read().unwrap();
    let root_nd_info = guard.nd_info.Clone();
    drop(guard);
    let mut index = 1;

    // tell to all other nodes to enter the network by contacting the root node
    while index < nodes.len() {
        if let Ok(mut guard) = nodes[index].write() {
            guard.connect(&root_nd_info);
            index += 1;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        else {
            println!("Failed to lock node");
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    println!("10 nodes have been initialized !");

    // Infint loop to keep the program running
    loop {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

}
