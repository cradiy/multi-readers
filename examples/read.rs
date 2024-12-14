use std::io::{Cursor, Read};

use multi_readers::wrap;

fn main() -> std::io::Result<()> {
    // Same type
    let r1 = Cursor::new("Hello, ");
    let r2 = Cursor::new("World!");
    let mut readers = wrap!(r1.clone(), r2.clone());
    let mut hello_world = String::new();
    readers.read_to_string(&mut hello_world)?;
    // Different types
    let r3 = Cursor::new(b" Rust!");
    assert_eq!(hello_world.as_str(), "Hello, World!");
    let mut readers = wrap!(dyn Read, r1, r2, r3);
    let mut buf = String::new();
    readers.read_to_string(&mut buf)?;
    assert_eq!(buf.as_str(), "Hello, World! Rust!");
    Ok(())
}
