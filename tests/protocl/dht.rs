use sha1::{Digest, Sha1};
use std::str;

#[test]
fn test_sha1() {
    let mut a = Sha1::new();
    let a = a.finalize().to_owned();
    println!("{:x}", a);
}
