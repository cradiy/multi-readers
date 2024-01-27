use std::io::Read;



/// A bytes wrapper and implements trait [`Read`][std::io::Read]
/// # Usage
/// ```
/// use multi_readers::BytesReader;
/// use std::io::Read;
/// let bytes = b"hello world";
/// let mut reader = BytesReader::new(bytes.to_vec());
/// let mut buf = [0; 6];
/// let size = reader.read(&mut buf).unwrap();
/// assert_eq!(&buf[..size], b"hello ");
/// let size = reader.read(&mut buf).unwrap();
/// assert_eq!(&buf[..size], b"world");
/// ```
/// 
pub struct BytesReader {
    buf: Vec<u8>,
    index: usize,
}

impl BytesReader {
    /// Create a new `BytesReader` with bytes
    pub fn new(buf: Vec<u8>) -> BytesReader {
        Self { buf, index: 0 }
    }
}
impl Read for BytesReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remain = self.buf.len() - self.index;
        if remain == 0 {
            return Ok(0);
        }
        if buf.len() >= remain {
            buf[..remain].copy_from_slice(&self.buf[self.index..]);
            self.index = self.buf.len();
            Ok(remain)
        } else {
            buf.copy_from_slice(&self.buf[self.index..self.index + buf.len()]);
            self.index += buf.len();
            Ok(buf.len())
        }
    }
}

/// Warpper for multiple readers
pub struct MultiReaders<'iter> {
    current: Option<Box<dyn Read>>,
    iter: Box<dyn Iterator<Item = Box<dyn Read>> + 'iter>,
}


#[allow(clippy::should_implement_trait)]
impl<'i> MultiReaders<'i> {
    /// Create a new `MultiReaders` from an iterator.
    pub fn from_iter(iter: impl Iterator<Item = Box<dyn Read>> + 'i) -> Self {
        Self {
            current: None,
            iter: Box::new(iter),
        }
    }
}

impl<'iter> Read for MultiReaders<'iter> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.current.is_none() {
            self.current = self.iter.next();
        }
        match &mut self.current {
            Some(r) => {
                let mut len = r.read(buf)?;
                if len == buf.len() {
                    return Ok(len);
                }
                self.current = self.iter.next();
                len += self.read(&mut buf[len..])?;
                Ok(len)
            }
            None => Ok(0)
        }
    }
}
