use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::{Arc, PoisonError, RwLock};
use std::time::Duration;
use std::time::Instant;
use std::vec;

use crate::request::{
    self, build_new_connection_reponse, build_new_connection_request, build_notify_request,
    tcpexchange_from_string, TCPExchange,
};

pub struct NodeInfo {
    pub id: u64,
    pub ip_addr: IpAddr, // TODO: Maybe convert to String
    pub port: u16,
    pub last_seen: Instant,
}

impl NodeInfo {
    pub fn Clone(&self) -> NodeInfo {
        NodeInfo {
            id: self.id,
            ip_addr: self.ip_addr,
            port: self.port,
            last_seen: self.last_seen,
        }
    }
}

pub struct Node {
    pub nd_info: NodeInfo,
    socket: Option<SocketAddr>, 
    kbucket: Box<Vec<NodeInfo>>, // TODO: use a better data structure
}

impl Node {
    pub fn new(nd_info: NodeInfo) -> Node {
        let socket = Option::None;
        let kbucket = Box::new(Vec::new());
        return Node {
            nd_info,
            socket,
            kbucket,
        };
    }

    /*
    Listen for incoming connections
     */
    fn listen(&mut self, tx: mpsc::Sender<String>) {
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
    }

    pub fn start(&mut self, tx: mpsc::Sender<String>) {
        self.listen(tx);
    }

    fn root(&mut self) {}

    /*
    Join a network by connecting to a node
     */
    pub fn connect(&mut self, nd_info: &NodeInfo) {
        self.socket = Some(SocketAddr::new(nd_info.ip_addr, nd_info.port));
        let request = build_new_connection_request(&self.nd_info);
        if let Ok(stream) =
            TcpStream::connect_timeout(&self.socket.unwrap(), Duration::from_secs(5))
        {
            println!(
                "[Node {}] Connected to node with id: {} , port: {}, ip: {} ",
                self.nd_info.id, nd_info.id, nd_info.port, nd_info.ip_addr
            );
            let mut stream = stream;
            stream.write(request.as_bytes()).unwrap();
        } else {
            println!(
                "[Node {}] Failed to connect to node with id: {} , port: {}, ip: {} ",
                self.nd_info.id, nd_info.id, nd_info.port, nd_info.ip_addr
            );
        }
    }

    /*
    This function is called when a new connection is established ( this node have successfully joined the newtork)
    TODO: Implement kademlia's metric for finding the closest nodes into this function
     */
    fn init(&mut self) {
        for node in self.kbucket.iter() {
            println!(
                "Node {} has node {} in its kbucket",
                self.nd_info.id, node.id
            );
        }
    }

    fn find_node(&mut self, id: u64) {}

    fn ping(&mut self, nd_info: NodeInfo) {}

    fn store(&mut self, key: u64, value: String) {}

    fn find_value(&mut self, key: u64) {}

    fn compute_xor_distance(&self, other: &Node) -> u64 {
        self.nd_info.id ^ other.nd_info.id
    }

    fn propagate(&mut self, key: u64, value: String) {}

    /*
    Notify nodes in the kbucket that a new node has joined the network
    */
    fn notify(&self, new_nd_info: &NodeInfo) {
        for node in self.kbucket.iter() {
            let request = build_notify_request(&self.nd_info, &new_nd_info);
            let ip = node.ip_addr.to_string();
            let port = node.port;
            self.send(request, ip, port);
        }
    }
    /*
    Send data to a node
     */
    fn send(&self, data: String, ip: String, port: u16) {
        let socket = SocketAddr::new(ip.parse().unwrap(), port);
        if let Ok(stream) = TcpStream::connect_timeout(&socket, Duration::from_secs(5)) {
            let mut stream = stream;
            stream.write(data.as_bytes()).unwrap();
        }
    }
    /*
    Handle incoming requests
     */
    pub fn handle_request(&mut self, request: String) {
        println!("[Node {}] Handling request {}", self.nd_info.id, request);
        let request_type;
        if let Ok(parsed) = json::parse(&request) {
            request_type = tcpexchange_from_string(parsed["type"].as_str().unwrap());
        } else {
            println!("Failed to parse request");
            return;
        }
        match request_type {
            TCPExchange::Connect => {
                self.handle_connect(request);
            }
            TCPExchange::RespConnect => {
                self.handle_respconnect(request);
            }
            TCPExchange::Notify => {
                self.handle_notify(request);
            }
            TCPExchange::Ping => todo!(),
            TCPExchange::FindNode => todo!(),
            TCPExchange::FoundNode => todo!(),
            TCPExchange::Store => todo!(),
            TCPExchange::Stored => todo!(),
            TCPExchange::FindValue => todo!(),
            TCPExchange::FoundValue => todo!(),
            TCPExchange::Error => todo!(),
        }
    }

    fn handle_notify(&mut self, request: String) {
        let parsed = json::parse(&request).unwrap();
        let new_nd_info = NodeInfo {
            id: parsed["new_node"]["id"].as_u64().unwrap(),
            ip_addr: IpAddr::from_str(parsed["new_node"]["ip_addr"].as_str().unwrap()).unwrap(),
            port: parsed["new_node"]["port"].as_u16().unwrap(),
            last_seen: Instant::now(),
        };
        self.kbucket.push(new_nd_info);
    }

    fn handle_respconnect(&mut self, request: String) {
        // TODO: must check if node is already in kbucket
        let kbucket = json::parse(&request).unwrap()["kbucket"].clone();
        for i in 0..kbucket.len() {
            let node = NodeInfo {
                id: kbucket[i]["id"].as_u64().unwrap(),
                ip_addr: kbucket[i]["ip_addr"].as_str().unwrap().parse().unwrap(),
                port: kbucket[i]["port"].as_u16().unwrap(),
                last_seen: Instant::now(),
            };
            self.kbucket.push(node);
        }
        self.init();
    }

    fn handle_connect(&mut self, request: String) {
        let parsed = json::parse(&request).unwrap();
        let ip = parsed["ip_addr"].as_str().unwrap().to_string();
        let port = parsed["port"].as_u16().unwrap();
        let id = parsed["id"].as_u64().unwrap();
        let response = build_new_connection_reponse(&self.nd_info, &self.kbucket);
        self.send(response, ip.clone(), port.clone());
        let nd_info = NodeInfo {
            id: id,
            ip_addr: IpAddr::from_str(ip.as_str()).unwrap(),
            port: port,
            last_seen: Instant::now(),
        };
        self.notify(&nd_info);
        self.kbucket.push(nd_info);
    }
}


/*
Helper function that start the listening thread and handle request fetch from the listening queue
 */
pub fn helper_start_node(shared_node: Arc<RwLock<Node>>) {
    std::thread::spawn(move || {
        println!(
            "Starting node handler; ip: {}, port: {}",
            shared_node.read().unwrap().nd_info.ip_addr,
            shared_node.read().unwrap().nd_info.port
        );
        let (tx, rx) = mpsc::channel();
        let mut guard = shared_node.write().expect("Failed to lock node");
        guard.start(tx);
        //release lock
        drop(guard);
        loop {
            // wait lock to be released
            let do_steps = || -> Result<(), PoisonError<RwLock<Node>>> {
                if let Ok(request) = rx.try_recv() {
                    let mut guard = shared_node.write().unwrap();
                    // remote null bytes
                    let request = request.trim_end_matches(char::from(0));
                    guard.handle_request(request.to_string());
                    //release lock
                    drop(guard);
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
                Ok(())
            };
            if let Err(e) = do_steps() {
                println!("Error: {}", e);
                // wait a bit
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        }
    });
}
