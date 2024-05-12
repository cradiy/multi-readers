<p>
    <a href="https://crates.io/crates/multi-readers">
    	<img alt="Crate Info" src="https://img.shields.io/crates/v/multi-readers.svg"/>
    </a>
</p>

# Multiple Readers

`multiple-readers ` is a Rust library aimed at simplifying the process of combining multiple types that implement the [std::io::Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html)  trait into a unified reader.

# Features

- Combines multiple types that implement the [std::io::Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html) trait into a unified reader.
- Provides [SliceReader](https://docs.rs/multi-readers/*/multi_readers/struct.SliceReader.html) and [BytesReader](https://docs.rs/multi-readers/*/multi_readers/struct.BytesReader.html) types, which respectively wrap `&[u8]` and `Vec<u8>`, implementing the [std::io::Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html) and
  [tokio::io::AsyncRead](https://docs.rs/tokio/*/tokio/io/trait.AsyncRead.html) trait.
- Can read from data sources sequentially until all data sources are exhausted.
- Supports [tokio](https://crates.io/crates/tokio) (` Unstable` )

# Example

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



# Async Example

- dependencies

```toml
tokio = { version = "*", features = ["full"]}
multi-readers = {version = "*", features = ["async"]}
```


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
