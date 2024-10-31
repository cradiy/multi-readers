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
