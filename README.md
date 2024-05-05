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

# Async
## Dependencies
```toml
tokio = {version = "*", features = ["full"]}
multi-readers = {version = "*", features = ["async"]}
```

## Example
```rust
use multi_readers::*;
use tokio::io::AsyncReadExt;
#[tokio::main]
async fn main() {
    let slice1 = SliceReader::new(b"12345");
    let slice2 = SliceReader::new(b"2346");
    let mut reader = join_async_readers!(slice1, slice2);
    let mut buf = [0; 4];
    let len = reader.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..len], b"1234");
    let len = reader.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..len], b"5234");
    let len = reader.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..len], b"6");
}
```



