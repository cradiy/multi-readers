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
    pub(crate) buf: Vec<u8>,
    pub(crate) pos: usize,
}
impl BytesReader {
    /// Create a new `BytesReader` with bytes
    pub fn new(buf: Vec<u8>) -> BytesReader {
        Self { buf, pos: 0 }
    }
    pub(crate) fn as_slice_reader(&self) -> SliceReader<'_> {
        SliceReader {
            buf: &self.buf,
            pos: self.pos
        }
    }
}
impl Read for BytesReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut slice_reader = self.as_slice_reader();
        let len = std::io::Read::read(&mut slice_reader, buf)?;
        self.pos = slice_reader.pos;
        Ok(len)
    }
}




/// A slice wrapper and implements trait [`Read`][std::io::Read]
/// # Usage
/// ```
/// use multi_readers::SliceReader;
/// use std::io::Read;
/// let slice = b"hello world";
/// let mut reader = SliceReader::new(slice);
/// let mut buf = [0; 6];
/// let size = reader.read(&mut buf).unwrap();
/// assert_eq!(&buf[..size], b"hello ");
/// let size = reader.read(&mut buf).unwrap();
/// assert_eq!(&buf[..size], b"world");
/// ```
///
pub struct SliceReader<'a> {
    pub(crate) buf: &'a [u8],
    pub(crate) pos: usize,
}

impl<'a> SliceReader<'a> {
    /// Create a new `SliceReader` with slice
    pub fn new(slice: &'a [u8]) -> SliceReader {
        Self { buf: slice, pos: 0 }
    }
}
impl<'a> Read for SliceReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remain = self.buf.len() - self.pos;
        if remain == 0 {
            return Ok(0);
        }
        if buf.len() >= remain {
            buf[..remain].copy_from_slice(&self.buf[self.pos..]);
            self.pos = self.buf.len();
            Ok(remain)
        } else {
            buf.copy_from_slice(&self.buf[self.pos..self.pos + buf.len()]);
            self.pos += buf.len();
            Ok(buf.len())
        }
    }
}

pub(crate) type HandleFunc<'func> = Option<Box<dyn Fn(&mut [u8]) + 'func>>;

/// Wrapper for multiple readers
///
/// `MultiReader` is lazy. It does nothing if you don't use.
pub struct MultiReaders<'iter, 'life, 'func> {
    current: Option<Box<dyn Read + 'life>>,
    iter: Box<dyn Iterator<Item = Box<dyn Read + 'life>> + 'iter>,
    process_func: HandleFunc<'func>,
}

#[allow(clippy::should_implement_trait)]
impl<'iter, 'life, 'func> MultiReaders<'iter, 'life, 'func> {
    /// Create a new `MultiReaders` from an iterator.
    pub fn from_iter(iter: impl Iterator<Item = Box<dyn Read + 'life>> + 'iter) -> Self {
        Self {
            current: None,
            iter: Box::new(iter),
            process_func: None,
        }
    }
    /// Process the bytes read using the given function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Read;
    /// use multi_readers::*;
    ///
    /// let bytes = b"01234567";
    /// let bytes1 = b"abcd";
    /// let mut reader = join_readers!(SliceReader::new(bytes), SliceReader::new(bytes1));
    /// reader.set_process_func(|slice| {
    ///     for b in slice {
    ///         *b += 1;
    ///     }
    /// });
    /// let mut buf = [0; 7];
    /// let len = reader.read(&mut buf).unwrap();
    /// assert_eq!(b"1234567", &buf[..len]);
    /// let len = reader.read(&mut buf).unwrap();
    /// assert_eq!(b"8bcde", &buf[..len]);
    /// ```
    pub fn set_process_func(&mut self, func: impl Fn(&mut [u8]) + 'func) {
        self.process_func = Some(Box::new(func))
    }
}

impl<'iter, 'life, 'func> Read for MultiReaders<'iter, 'life, 'func> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.current.is_none() {
            self.current = self.iter.next();
        }
        match &mut self.current {
            Some(r) => {
                let mut len = r.read(buf)?;
                if let Some(f) = &self.process_func {
                    f(&mut buf[..len]);
                }
                if len < buf.len() {
                    self.current = self.iter.next();
                    len += self.read(&mut buf[len..])?;
                }
                Ok(len)
            }
            None => Ok(0),
        }
    }
}

