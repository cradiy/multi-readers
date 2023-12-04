Merge multiple streams(R: std::io::Read + std::io::Seek) into one stream

# Example
```rust
use multi_stream::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    std::fs::write("1", b"Hello ")?;
    std::fs::write("2", b"\nRust")?;
    let mut r = MultiReaders::new();
    r.push(File::open("1")?)?;
    r.push(File::open("2")?)?;
    let mut buf_reader = BufReader::new(r);
    let mut buf = String::new();
    buf_reader.read_line(&mut buf)?;
    assert_eq!(buf.as_bytes(), b"Hello \n");
    buf.clear();
    buf_reader.read_line(&mut buf)?;
    assert_eq!(buf.as_bytes(), b"Rust");
    Ok(())
}


```