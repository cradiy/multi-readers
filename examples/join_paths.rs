use std::io::Read;

use multi_readers::join_paths_to_readers;
fn main() {
    let mut reader = join_paths_to_readers!("Cargo.toml", "src/lib.rs").unwrap();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
}