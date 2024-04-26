use std::io::Read;
use std::io::Result;
/// An extension trait for [`Read`][std::io::Read]
pub trait ExRead: Read {
    /// Pull some bytes from this source into vec
    /// 
    /// # Example
    /// ```
    /// use multi_readers::*;
    /// let mut reader = SliceReader::new(b"Hello World");
    /// let buf = reader.read_to_vec(5).unwrap();
    /// assert_eq!(buf.as_slice(), b"Hello");
    /// let buf = reader.read_to_vec(7).unwrap();
    /// assert_eq!(buf.len(), 6);
    /// assert_eq!(buf.as_slice(), b" World");
    /// ```
    fn read_to_vec(&mut self, size: usize) -> Result<Vec<u8>>;
    /// Read the exact number of bytes required to fill buf
    /// 
    /// # Example
    /// ```rust
    /// use multi_readers::*;
    /// let mut reader = SliceReader::new(b"Hello World");
    /// let buf = reader.read_exact_to_vec(5).unwrap();
    /// assert_eq!(buf.as_slice(), b"Hello");
    /// // cannot fill buf
    /// let result = reader.read_exact_to_vec(7);
    /// assert!(result.is_err())
    /// ```
    fn read_exact_to_vec(&mut self, size: usize) -> Result<Vec<u8>>;
}

impl<R: Read> ExRead for R {
    fn read_to_vec(&mut self, size: usize) -> Result<Vec<u8>> {
        if size == 0 {
            return Ok(Vec::new())
        }
        let mut buf = Vec::with_capacity(size);
        read_to_vec(self, size, &mut buf, false)?;
        Ok(buf)
    }

    fn read_exact_to_vec(&mut self, size: usize) -> Result<Vec<u8>> {
        if size == 0 {
            return Ok(Vec::new());
        }
        let mut buf = Vec::with_capacity(size);
        read_to_vec(self, size, &mut buf, true)?;
        Ok(buf)
    }
}

macro_rules! read {
    ($len:expr,$r:expr, $size:expr, $vec:expr, $exact:expr) => {{
        let count = $size / $len;
        #[allow(clippy::modulo_one)]
        let exp = $size % $len;
        let mut buf = [0; $len];
        for _ in 0..count {
            let _size = if $exact {
                $r.read_exact(&mut buf)?;
                $len
            } else {
                let size = $r.read(&mut buf)?;
                size
            };
            $vec.extend_from_slice(&buf[.._size]);
        }
        if exp != 0 {
            read_to_vec($r, exp, $vec, $exact)?;
        }
    }};
}

fn read_to_vec<R: Read>(r: &mut R, size: usize, buf: &mut Vec<u8>, exact: bool) -> Result<()> {
    match size {
        0 => (),
        1..=32 => read!(1, r, size, buf, exact),
        33..=1024 => read!(32, r, size, buf, exact),
        _ => read!(1024, r, size, buf, exact),
    }
    Ok(())
}
