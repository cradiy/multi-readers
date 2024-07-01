use std::io::Read;
use std::io::Result;
macro_rules! append_buffer {
    ($len:expr,$r:expr, $size:expr, $vec:expr) => {{
        let count = $size / $len;
        let mut buf = [0; $len];
        for _ in 0..count {
            let len = $r.read(&mut buf)?;
            if len == 0 {
                return Ok(());
            }
            $vec.extend_from_slice(&buf[..len]);
        }
    }};
}

#[inline]
fn _read<R: Read>(r: &mut R, vec: &mut Vec<u8>) -> Result<()> {
    while vec.len() < vec.capacity() {
        let size = vec.capacity() - vec.len();
        match size {
            0 => return Ok(()),
            1..=31 => append_buffer!(1, r, size, vec),
            32..=1023 => append_buffer!(32, r, size, vec),
            _ => append_buffer!(1024, r, size, vec),
        };
    }
    Ok(())
}

/// Reads a specified size of bytes from the source into a Vec.
///
/// # Example
/// ```
/// use multi_readers::{read, SliceReader};
/// use std::io::Read;
/// let mut r = SliceReader::new(b"Hello");
/// let buf = read(&mut r, 4).unwrap();
/// assert_eq!(buf.len(), 4);
/// let buf = read(&mut r, 4).unwrap();
/// assert_eq!(buf.len(), 1);
/// ```
pub fn read<R: Read>(r: &mut R, cap: usize) -> Result<Vec<u8>> {
    if cap == 0 {
        return Ok(Vec::new());
    }
    let mut buf = Vec::with_capacity(cap);
    _read(r, &mut buf)?;
    Ok(buf)
}



#[test]
fn test() {
    use crate::SliceReader;
    let mut r = SliceReader::new(b"Hello");
    let buf = read(&mut r, 3).unwrap();
    println!("{:?}", buf);
    let buf = read(&mut r, 3).unwrap();
    println!("{:?}", buf);
}
