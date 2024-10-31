<p>
    <a href="https://crates.io/crates/multi-readers">
    	<img alt="Crate Info" src="https://img.shields.io/crates/v/multi-readers.svg"/>
    </a>
</p>

# Multiple Readers

`multiple-readers ` is a Rust library aimed at simplifying the process of combining multiple types that implement the [std::io::Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html)  trait into a unified reader.

# Features

- Combines multiple types that implement the [std::io::Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html) trait into a unified reader.
- Can read from data sources sequentially until all data sources are exhausted.
- Supports [tokio](https://crates.io/crates/tokio) (` Unstable` )

# Example

```rust
use multi_readers::join_readers;
use std::io::{Cursor, Read};
fn main() -> std::io::Result<()> {

    let slice = Cursor::new(b"First-");
    let bytes = Cursor::new(b"Second-");
    let mut reader = join_readers!(slice, bytes);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    assert_eq!(buf.as_str(), "First-Second-");
    Ok(())
}

```
