use std::{fs::File, io::Read};

use multi_readers::join_readers;

fn main() {
    let f1 = File::open("Cargo.toml").unwrap();
    let f2 = File::open("src/lib.rs").unwrap();
    let mut readers = join_readers!(f1, f2);
    let mut buf = String::new();
    readers.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
}
