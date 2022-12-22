use crate::errors::{Error, Result};
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Display;
use std::net::SocketAddr;

const BUCKET_SIZE: usize = 8;
const KEY_LENGTH: usize = 20;
const KEY_SPACE: usize = 160;
const MAX_PREFIX_LENGTH: usize = 10;

#[derive(Default)]
struct Trie {
    bucket: Bucket,
    left: Option<Box<Trie>>,
    right: Option<Box<Trie>>,
}

impl Trie {
    fn insert(&mut self, node: Node, i: usize) -> bool {
        let root = if node.id.bit(i) == 0 {
            &mut self.right
        } else {
            &mut self.left
        };
        match root {
            Some(next) => next.insert(node, i + 1),
            None => {
                if self.bucket.is_full() {
                    self.split();
                    self.insert(node, i)
                } else {
                    self.bucket.add(node)
                }
            }
        }
    }

    fn get(&self, id: &Key, i: usize) -> Option<&Node> {
        let root = if id.bit(i) == 0 {
            &self.right
        } else {
            &self.left
        };
        match root {
            Some(next) => next.get(id, i + 1),
            None => self.bucket.nodes.get(id),
        }
    }

    fn split(&mut self) -> bool {
        todo!()
    }
}

pub struct RouteTable {
    self_node: Node,
    node_num: usize,
    root: Box<Trie>,
}

impl RouteTable {
    pub fn new(addr: &str) -> Result<Self> {
        Ok(Self {
            self_node: Node {
                id: Key::new(),
                addr: addr.parse()?,
            },
            node_num: 0,
            root: Box::new(Trie::default()),
        })
    }

    pub fn put(&mut self, node: Node) -> Result<()> {
        self.root.insert(node, 0);
        self.node_num += 1;

        Ok(())
    }

    pub fn get(&self, id: &Key) -> Option<&Node> {
        self.root.get(id, 0)
    }

    pub(crate) fn distance(&self, node: &Node) -> u64 {
        todo!()
    }
}

#[derive(Default)]
pub struct Bucket {
    last_changed: u64,
    nodes: HashMap<Key, Node>,
}

impl Bucket {
    fn is_full(&self) -> bool {
        BUCKET_SIZE == self.nodes.len()
    }

    fn add(&mut self, node: Node) -> bool {
        if self.is_full() {
            return false;
        }
        self.nodes.insert(node.id.clone(), node);
        true
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    id: Key,
    addr: SocketAddr,
}

impl Node {
    pub fn new(id: &str, addr: &str) -> Result<Self> {
        Ok(Node {
            id: id.try_into()?,
            addr: addr.parse()?,
        })
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Node {{ id: '{}' addr: '{}' }}",
            self.id.to_string(),
            self.addr
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Key {
    data: [u8; KEY_LENGTH],
}

impl Key {
    pub fn new() -> Key {
        let a: [u8; KEY_LENGTH] = Sha1::new().finalize().into();
        a.into()
    }

    pub fn bit(&self, i: usize) -> u8 {
        let div = i << 3;
        self.data[div] & 1 << 7
    }

    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

impl TryFrom<&str> for Key {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() != KEY_LENGTH {
            return Err(Error::InvalidKey(value.to_string()));
        }
        Ok(Self {
            data: value.as_bytes().try_into()?,
        })
    }
}

impl From<[u8; KEY_LENGTH]> for Key {
    fn from(data: [u8; KEY_LENGTH]) -> Self {
        Self { data }
    }
}
