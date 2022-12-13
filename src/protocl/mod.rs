use std::collections::HashMap;
use util::bencode;

use crate::{
    errors::Error,
    errors::Result,
    util::{self, bencode::Value},
};

#[derive(Debug, PartialEq)]
pub enum DHTQuery {
    Ping {
        id: String,
    },
    FindNode {
        id: String,
        target: String,
    },
    GetPeers {
        id: String,
        info_hash: String,
    },
    AnnouncePeer {
        id: String,
        impiled_port: u8,
        port: u64,
        info_hash: String,
        token: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum DHTResponse {
    ID {
        id: String,
    },
    FindNode {
        id: String,
        nodes: String,
    },
    GetPeers {
        id: String,
        token: String,
        values: Vec<String>,
    },
}

#[derive(Debug, PartialEq)]
pub enum KRPC {
    Query(DHTQuery),
    Response(DHTResponse),
    Error(u64, String),
}

impl KRPC {
    pub fn decode(s: &str) -> Result<Self> {
        match bencode::decode(&mut s.chars().peekable())? {
            bencode::Value::Dict(ref mut dict) => match dict.get("y") {
                Some(Value::String(q)) if q == "q" => Self::decode_query(dict),
                Some(Value::String(e)) if e == "e" => Self::decode_error(dict),
                Some(Value::String(r)) if r == "r" => Self::decode_response(dict),
                _ => Err(Error::InvalidKRPC),
            },
            _ => Err(Error::InvalidKRPC),
        }
    }

    fn decode_query(m: &mut HashMap<String, Value>) -> Result<Self> {
        if let Some(Value::Dict(mut a)) = m.remove("a") {
            return match m.get("q") {
                Some(Value::String(q)) if q == "ping" => {
                    if let Some(Value::String(id)) = a.remove("id") {
                        return Ok(Self::Query(DHTQuery::Ping { id }));
                    }
                    Err(Error::InvalidKRPC)
                }
                Some(Value::String(q)) if q == "find_node" => {
                    let id = a.remove("id");
                    let target = a.remove("target");
                    if id.is_none() || target.is_none() {
                        return Err(Error::InvalidKRPC);
                    }
                    Ok(Self::Query(DHTQuery::FindNode {
                        id: id.unwrap().try_into()?,
                        target: target.unwrap().try_into()?,
                    }))
                }
                Some(Value::String(q)) if q == "announce_peer" => {
                    let id = a.remove("id");
                    let token = a.remove("token");
                    let info_hash = a.remove("info_hash");
                    let port = a.remove("port");
                    let implied_port = a.remove("implied_port");
                    if id.is_none()
                        || token.is_none()
                        || info_hash.is_none()
                        || port.is_none()
                        || implied_port.is_none()
                    {
                        return Err(Error::InvalidKRPC);
                    }
                    Ok(Self::Query(DHTQuery::AnnouncePeer {
                        id: id.unwrap().try_into()?,
                        impiled_port: implied_port.unwrap().try_into()?,
                        port: port.unwrap().try_into()?,
                        info_hash: info_hash.unwrap().try_into()?,
                        token: token.unwrap().try_into()?,
                    }))
                }
                Some(Value::String(q)) if q == "get_peers" => {
                    let id = a.remove("id");
                    let info_hash = a.remove("info_hash");
                    if id.is_none() || info_hash.is_none() {
                        return Err(Error::InvalidKRPC);
                    }
                    Ok(Self::Query(DHTQuery::GetPeers {
                        id: id.unwrap().try_into()?,
                        info_hash: info_hash.unwrap().try_into()?,
                    }))
                }
                _ => Err(Error::InvalidKRPC),
            };
        }
        Err(Error::InvalidKRPC)
    }

    fn decode_error(m: &mut HashMap<String, Value>) -> Result<Self> {
        if let Some(Value::List(ref mut list)) = m.remove("e") {
            if list.len() == 2 {
                return Ok(Self::Error(
                    list.remove(0).try_into()?,
                    list.remove(0).try_into()?,
                ));
            }
        }
        Err(Error::InvalidKRPC)
    }

    fn decode_response(m: &mut HashMap<String, Value>) -> Result<Self> {
        if let Some(Value::Dict(ref mut dict)) = m.remove("r") {
            // maybe id response
            let id = dict.remove("id");
            if id.is_none() {
                return Err(Error::InvalidKRPC);
            }
            // find_nodes
            if let Some(nodes) = dict.remove("nodes") {
                return Ok(Self::Response(DHTResponse::FindNode {
                    id: id.unwrap().try_into()?,
                    nodes: nodes.try_into()?,
                }));
            }
            // get_peers
            if let Some(values) = dict.remove("values") {
                if let Some(token) = dict.remove("token") {
                    return Ok(Self::Response(DHTResponse::GetPeers {
                        id: id.unwrap().try_into()?,
                        token: token.try_into()?,
                        values: values.try_into()?,
                    }));
                }
            }

            return Ok(Self::Response(DHTResponse::ID {
                id: id.unwrap().try_into()?,
            }));
        }
        Err(Error::InvalidKRPC)
    }
}
