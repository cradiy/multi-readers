use std::io::prelude::Read;

use multi_readers::{join_readers, SliceReader};

fn main() {
    let bytes = b"12345678";
    let bytes1 = b"12345678";
    let mut reader = join_readers!(SliceReader::new(bytes), SliceReader::new(bytes1));
    reader.set_process_func(|slice| {
        for b in slice {
            *b += 1
        }
    });
    let mut buf = [0; 7];
    let len = reader.read(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf[..len]));
    let len = reader.read(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf[..len]));
    let len = reader.read(&mut buf).unwrap();
    println!("{:?}", String::from_utf8_lossy(&buf[..len]));
}
