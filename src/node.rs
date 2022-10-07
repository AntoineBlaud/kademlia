



use std::io::{Write, Read};
use std::net::{IpAddr, TcpStream, TcpListener};
use std::sync::mpsc;
use std::time::Instant;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock, PoisonError};
use std::time::Duration;


use crate::request::{build_new_connection_request, TCPExchange};



pub struct NodeInfo {
    pub id: u64,
    pub ip_addr: IpAddr,
    pub port: u16,
    pub last_seen: Instant
}

impl NodeInfo {
    pub fn Clone(&self) -> NodeInfo {
        NodeInfo {
            id: self.id,
            ip_addr: self.ip_addr,
            port: self.port,
            last_seen: self.last_seen
        }
    }
}

pub struct Node {
    pub nd_info : NodeInfo,
    socket: Option<SocketAddr>,
    kbucket: Box<Vec<Vec<Vec<()>>>>
}

impl Node {

    pub fn new(nd_info : NodeInfo) -> Node  {
        let socket = Option::None;
        let kbucket = Box::new(Vec::new());
        return Node {nd_info, socket, kbucket};
    }

    pub fn connect(&mut self, nd_info: & NodeInfo) {
        println!("[Node {}] Connecting to node with id: {}", self.nd_info.id, nd_info.id);
        self.socket = Some(SocketAddr::new(nd_info.ip_addr, nd_info.port));
        let request = build_new_connection_request(nd_info);
        if let Ok(stream) = TcpStream::connect_timeout(&self.socket.unwrap(), Duration::from_secs(5)) {
            println!("[Node {}] Connected to node with id: {}", self.nd_info.id, nd_info.id);
            let mut stream = stream;
            stream.write(request.as_bytes()).unwrap();
        } else {
            println!("[Node {}] Failed to connect to node with id: {}", self.nd_info.id, nd_info.id);
        }
        
    }

    fn find_node(&mut self, id: u64){

    }

    fn ping(&mut self, nd_info : NodeInfo) {

    }

    fn store(&mut self, key: u64, value: String) {

    }

    fn find_value(&mut self, key: u64) {

    }

    fn compute_xor_distance(&self, other: &Node) -> u64 {
        self.nd_info.id ^ other.nd_info.id
    }
    

    fn propagate(&mut self, key: u64, value: String) {

    }
    fn notify(&mut self, nd_info: NodeInfo) {

    }
    pub fn handle_request(&mut self, request: String) {
        println!("[Node {}] Received request: {}", self.nd_info.id, request);

    }
    fn listen(&mut self) -> mpsc::Receiver<String> {
        let (tx, rx) = mpsc::channel();
        let ip = self.nd_info.ip_addr;
        let port = self.nd_info.port;
        println!("Listening on {}:{}", ip, port);
        std::thread::spawn(move || {
            let addr_str = format!("{}:{}", ip, port);
            let listener = TcpListener::bind(addr_str).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let mut stream = stream;
                        let mut buffer = [0; 4096];
                        stream.read(&mut buffer).unwrap();
                        let request = String::from_utf8_lossy(&buffer[..]);
                        tx.send(request.to_string()).unwrap();
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            
        });
        return rx;

    }

    pub fn start(&mut self) -> mpsc::Receiver<String> {
        return self.listen();
    }


    fn root(&mut self) {

    }
}



pub fn helper_start_node(shared_node : Arc<RwLock<Node>>) {
    std::thread::spawn(move || {
        println!("Starting node");
        let mut guard = shared_node.write().expect("Failed to lock node");
        let rx  = guard.start();
        //release lock
        drop(guard);
        loop {
            // wait lock to be released
            let do_steps = || -> Result<(), PoisonError<RwLock<Node>>> {
                let mut guard = shared_node.write().unwrap();
                if let Ok(request) = rx.try_recv() {
                    println!("Received message: {}", request);
                    guard.handle_request(request);
                }
                //release lock
                drop(guard);
                std::thread::sleep(std::time::Duration::from_millis(20));
                Ok(())
            };
            if let Err(e) = do_steps() {
                println!("Error: {}", e);
                // wait a bit
                std::thread::sleep(std::time::Duration::from_millis(300));
            }
        }        
    });
}