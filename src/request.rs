use std::fmt;

use crate::node::NodeInfo;


extern crate json;

pub enum TCPExchange {
    Connect,
    RespConnect,
    Ping,
    Notify,
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
            TCPExchange::RespConnect => write!(f, "RespConnect"),
            TCPExchange::Ping => write!(f, "Ping"),
            TCPExchange::Notify => write!(f, "Notify"),
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

pub fn tcpexchange_from_string(s: &str) -> TCPExchange {
    match s {
        "Connect" => TCPExchange::Connect,
        "RespConnect" => TCPExchange::RespConnect,
        "Ping" => TCPExchange::Ping,
        "Notify" => TCPExchange::Notify,
        "FindNode" => TCPExchange::FindNode,
        "FoundNode" => TCPExchange::FoundNode,
        "Store" => TCPExchange::Store,
        "Stored" => TCPExchange::Stored,
        "FindValue" => TCPExchange::FindValue,
        "FoundValue" => TCPExchange::FoundValue,
        "Error" => TCPExchange::Error,
        _ => panic!("Invalid TCPExchange"),
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

pub fn build_new_connection_reponse(nd_info : &NodeInfo, kbuket : &Box<Vec<NodeInfo>>) -> String {
    let mut request = json::JsonValue::new_object();
    _insert_node_info(&mut request, nd_info);
    request["type"] = TCPExchange::RespConnect.to_string().into();
    request["kbucket"] = json::JsonValue::new_array();
    for node in kbuket.iter() {
        let mut node_json = json::JsonValue::new_object();
        _insert_node_info(&mut node_json, node);
        request["kbucket"].push(node_json).unwrap();
    }
    request.dump()
}

pub fn build_notify_request(nd_info : &NodeInfo, new_nd_info:&NodeInfo ) -> String {
    let mut request = json::JsonValue::new_object();
    _insert_node_info(&mut request, nd_info);
    request["type"] = TCPExchange::Notify.to_string().into();
    request["new_node"] = json::JsonValue::new_object();
    _insert_node_info(&mut request["new_node"], new_nd_info);
    request.dump()
}