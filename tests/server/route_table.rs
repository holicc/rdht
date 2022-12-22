use rdht::errors::Result;
use rdht::server::route_table::{Key, Node, RouteTable};
use sha1::{Digest, Sha1};

#[test]
fn test_route_table_insert() -> Result<()> {
    let mut table = RouteTable::new("127.0.0.1:7891")?;
    let id = "12345678900987654321";
    table.put(Node::new(id, "127.0.0.1:8921")?)?;
    let node = table
        .get(&id.try_into()?)
        .expect("should get node from the table");
    println!("{}", node);
    Ok(())
}
