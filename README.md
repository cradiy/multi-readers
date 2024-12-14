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
use std::io::{Cursor, Read};

use multi_readers::wrap;

fn main() -> std::io::Result<()> {
    // Same type
    let r1 = Cursor::new("Hello, ");
    let r2 = Cursor::new("World!");
    let mut readers = wrap!(r1.clone(), r2.clone());
    let mut hello_world = String::new();
    readers.read_to_string(&mut hello_world)?;
    assert_eq!(hello_world.as_str(), "Hello, World!");
    // Different types
    let r3 = Cursor::new(b" Rust!");
    let mut readers = wrap!(dyn Read, r1, r2, r3);
    let mut buf = String::new();
    readers.read_to_string(&mut buf)?;
    assert_eq!(buf.as_str(), "Hello, World! Rust!");
    Ok(())
}

```
