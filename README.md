# Wrapper for multiple readers

`MultiReader` is lazy. It does nothing if you don't use.

## Usage
- `SliceReader` and `BytesReader`
```rust
use multi_readers::{BytesReader, SliceReader, join_readers};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let slice = SliceReader::new(b"hello");
    let bytes = BytesReader::new("world".as_bytes().to_vec()); 
    let mut reader = join_readers!(slice, bytes);
    let mut buf = [0; 5];
    let len = reader.read(&mut buf)?;
    assert_eq!(b"hello", &buf[..len]);
    let len = reader.read(&mut buf)?;
    assert_eq!(b"world", &buf[..len]);
    Ok(())
}
```

## Usage
- Merge any type that implements the trait `std::io::Read`
```rust
use multi_readers::{BytesReader, SliceReader, join_readers};
use std::{fs::File, io::Read};
fn main() -> std::io::Result<()> {
    let slice = SliceReader::new(b"First-");
    let bytes = BytesReader::new(b"Second-".to_vec()); 
    std::fs::write("test.txt", b"Third")?;
    let f = File::open("test.txt")?;
    let mut reader = join_readers!(slice, bytes, f);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    assert_eq!(buf.as_str(), "First-Second-Third");
    Ok(())
}


```