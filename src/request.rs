use std::fmt;

use crate::node::NodeInfo;


extern crate json;

pub enum TCPExchange {
    Connect,
    Ping,
    Pong,
    FindNode,
    FoundNode,
    Store,
    Stored,
    FindValue,
    FoundValue,
    Error,
}
impl fmt::Display for TCPExchange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TCPExchange::Connect => write!(f, "Connect"),
            TCPExchange::Ping => write!(f, "Ping"),
            TCPExchange::Pong => write!(f, "Pong"),
            TCPExchange::FindNode => write!(f, "FindNode"),
            TCPExchange::FoundNode => write!(f, "FoundNode"),
            TCPExchange::Store => write!(f, "Store"),
            TCPExchange::Stored => write!(f, "Stored"),
            TCPExchange::FindValue => write!(f, "FindValue"),
            TCPExchange::FoundValue => write!(f, "FoundValue"),
            TCPExchange::Error => write!(f, "Error"),
        }
    }
}


fn _insert_node_info(json: &mut json::JsonValue, nd_info: &NodeInfo)  {
    json["id"] = nd_info.id.into();
    json["ip_addr"] = nd_info.ip_addr.to_string().into();
    json["port"] = nd_info.port.into();
}


pub fn build_new_connection_request(nd_info : &NodeInfo) -> String {
    let mut request = json::JsonValue::new_object();
    _insert_node_info(&mut request, nd_info);
    request["type"] = TCPExchange::Connect.to_string().into();
    request.dump()
}