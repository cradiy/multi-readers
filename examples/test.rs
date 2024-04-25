use multi_readers::{join_readers, BytesReader, SliceReader};
use std::{fs::File, io::Read};
fn main() -> std::io::Result<()> {
    let slice = SliceReader::new(b"hello");
    let bytes = BytesReader::new(b"world".to_vec());
    let mut reader = join_readers!(slice, bytes);
    let mut buf = [0; 5];
    let len = reader.read(&mut buf)?;
    assert_eq!(b"hello", &buf[..len]);
    let len = reader.read(&mut buf)?;
    assert_eq!(b"world", &buf[..len]);

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
