
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, RwLock, PoisonError};
use rand::random;


mod node;
mod request;

use crate::node::{Node, NodeInfo, helper_start_node};



fn main() {
    let mut nodes = Vec::new();
    for i in 0..10 {
        let id = rand::random::<u64>();
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = (rand::random::<u16>()% 50000 + 1000); 
        let nd_info = NodeInfo {id, ip_addr, port, last_seen : std::time::Instant::now()};
        nodes.push(Arc::new(RwLock::new(Node::new(nd_info))));
    }
    for i in 0..10 {
        helper_start_node(nodes[i].clone());
    }
    println!("10 nodes have been initialized");

    let root_node = nodes.pop().unwrap();
    let guard = root_node.read().unwrap();
    let root_nd_info = guard.nd_info.Clone();
    let mut index = 1;


    std::thread::sleep(std::time::Duration::from_millis(100));
    while index < nodes.len() {
        if let Ok(mut guard) = nodes[index].write() {
            guard.connect(&root_nd_info);
        }
        else {
            println!("Failed to lock node");
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        index += 1;
    }

}
