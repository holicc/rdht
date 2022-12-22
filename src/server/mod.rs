use std::collections::HashSet;

use self::route_table::RouteTable;
use crate::errors::Result;

pub mod route_table;

pub struct Server {
    table: RouteTable,
    trackers: HashSet<String>,
}

impl Server {
    pub fn new(addr: &str, trackers: Vec<String>) -> Result<Self> {
        Ok(Server {
            table: RouteTable::new(addr)?,
            trackers: trackers.into_iter().collect(),
        })
    }

    pub fn run(&self) -> Result<()> {
        todo!()
    }

    pub fn refersh(&self) -> Result<()> {
        todo!()
    }

    pub fn find_node(&self) -> Result<()> {
        todo!()
    }

    pub fn get_peers(&self) -> Result<()> {
        todo!()
    }

    pub fn announce_peer(&self) -> Result<()> {
        todo!()
    }

    pub fn ping(&self) -> Result<()> {
        todo!()
    }
}
