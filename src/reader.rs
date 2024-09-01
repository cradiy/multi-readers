use std::{
    io::{Read, Seek},
    ops::Deref,
};

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
            pos: self.pos,
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
impl Seek for BytesReader {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let mut slice_reader = self.as_slice_reader();
        let len = slice_reader.seek(pos)?;
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

impl<'a> Deref for SliceReader<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.buf
    }
}

impl<'a> Seek for SliceReader<'a> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match pos {
            std::io::SeekFrom::Start(s) => {
                let pos = s.min(self.buf.len() as u64);
                self.pos = pos as usize;
                Ok(pos)
            }
            std::io::SeekFrom::End(e) => {
                let pos = if e >= 0 {
                    self.buf.len() as u64
                } else {
                    let pos = self.buf.len() as i64 + e;
                    0.max(pos) as u64
                };
                self.pos = pos as usize;
                Ok(pos)
            }
            std::io::SeekFrom::Current(c) => {
                let len = self.buf.len() as i64;
                let pos = 0.max(len + c).min(len) as u64;
                self.pos = pos as usize;
                Ok(pos)
            }
        }
    }
}

impl<'a> SliceReader<'a> {
    /// Create a new `SliceReader` with slice
    pub fn new(slice: &'a [u8]) -> SliceReader {
        Self { buf: slice, pos: 0 }
    }
}
impl<'a> From<&'a str> for SliceReader<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for SliceReader<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::new(value)
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
