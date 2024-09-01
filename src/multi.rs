use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

use crate::SeekRead;

/// Wrapper for multiple readers
///
/// `MultiReader` is lazy. It does nothing if you don't use.
pub struct MultiReaders<'iter, 'life> {
    current: Option<Box<dyn Read + 'life>>,
    iter: Box<dyn Iterator<Item = Box<dyn Read + 'life>> + 'iter>,
}

#[allow(clippy::should_implement_trait)]
impl<'iter, 'life> MultiReaders<'iter, 'life> {
    /// Create a new `MultiReaders` from an iterator.
    pub fn from_iter(iter: impl Iterator<Item = Box<dyn Read + 'life>> + 'iter) -> Self {
        Self {
            iter: Box::new(iter),
            current: None,
        }
    }
    pub fn from_vec<T: Read + 'life + 'iter, V: Into<Vec<T>>>(vec: V) -> Self {
        let vec: Vec<T> = vec.into();
        Self::from_iter(vec.into_iter().map(|v| Box::new(v) as Box<dyn Read>))
    }
}

impl<'iter, 'life> Read for MultiReaders<'iter, 'life> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.current.is_none() {
            self.current = self.iter.next();
        }
        match &mut self.current {
            Some(r) => {
                let mut len = r.read(buf)?;
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

struct SeekReader {
    data: Box<dyn SeekRead>,
    len: u64,
}

impl SeekReader {
    fn new(mut data: Box<dyn SeekRead>) -> std::io::Result<Self> {
        let len = data.seek(SeekFrom::End(0))?;
        data.seek(SeekFrom::Start(0))?;
        Ok(Self { data, len })
    }
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Default)]
pub struct MultiSeekReaders {
    buf: Vec<SeekReader>,
    pos: usize,
    len: u64,
}

impl MultiSeekReaders {
    pub fn new(readers: Vec<Box<dyn SeekRead>>) -> std::io::Result<Self> {
        if readers.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "readers cannot be empty!",
            ));
        }
        let mut buf = Vec::with_capacity(readers.len());
        let mut len = 0;
        for r in readers {
            let r = SeekReader::new(r)?;
            if !r.is_empty() {
                len += r.len;
                buf.push(r);
            }
        }
        Ok(Self { buf, pos: 0, len })
    }
    pub fn push(&mut self, value: impl SeekRead + 'static) -> std::io::Result<()> {
        let r = SeekReader::new(Box::new(value))?;
        if !r.is_empty() {
            self.buf.push(r);
        }
        Ok(())
    }
    pub fn multi_stream_len(&self) -> u64 {
        self.len
    }
}
impl Seek for MultiSeekReaders {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(s) => seek_start(self, s),
            SeekFrom::End(e) => seek_end(self, e),
            SeekFrom::Current(c) => seek_current(self, c),
        }
    }
    fn stream_position(&mut self) -> std::io::Result<u64> {
        let pos = self
            .buf
            .iter()
            .take(self.pos + 1)
            .fold(0, |out, v| out + v.len);
        let current = self.buf[self.pos].data.stream_position()?;
        Ok(pos + current)
    }
    fn rewind(&mut self) -> std::io::Result<()> {
        self.buf[0].data.rewind()?;
        self.pos = 0;
        Ok(())
    }
}
fn seek_start(seeker: &mut MultiSeekReaders, start: u64) -> std::io::Result<u64> {
    if start == seeker.len {
        seek_to_end(seeker)
    } else if start == 0 {
        seeker.rewind()?;
        Ok(0)
    } else if start > seeker.len {
        Err(Error::new(ErrorKind::InvalidInput, "Invalid seek"))
    } else {
        let mut count = 0;
        for i in 0..seeker.buf.len() {
            let len = seeker.buf[i].len;
            if (count..count + len).contains(&start) {
                let remain = start - count;
                seeker.buf[i].data.seek(SeekFrom::Start(remain))?;
                seeker.pos = i;
                return Ok(start);
            } else {
                count += len;
            }
        }
        unreachable!()
    }
}

fn seek_to_end(seeker: &mut MultiSeekReaders) -> std::io::Result<u64> {
    let pos = seeker.buf.len() - 1;
    seeker.buf[pos].data.seek(SeekFrom::End(0))?;
    seeker.pos = pos;
    Ok(seeker.len)
}

fn seek_end(seeker: &mut MultiSeekReaders, end: i64) -> std::io::Result<u64> {
    use std::cmp::Ordering::*;
    match end.cmp(&0) {
        Less => {
            let offset = seeker.len as i64 + end;
            if offset < 0 {
                Err(invalid_input("Invalid seek end"))
            } else {
                seek_start(seeker, offset as u64)
            }
        }
        Equal => seek_to_end(seeker),
        Greater => Err(invalid_input("Invalid end")),
    }
}
fn invalid_input(error: &str) -> Error {
    Error::new(ErrorKind::InvalidInput, error)
}
fn seek_current(seeker: &mut MultiSeekReaders, current: i64) -> std::io::Result<u64> {
    use std::cmp::Ordering::*;
    match current.cmp(&0) {
        Less => {
            let pos = seeker.stream_position()? as i64 + current;
            if pos < 0 {
                return Err(invalid_input("Invalid seek"));
            }
            seek_start(seeker, pos as u64)
        }
        Equal => seeker.stream_position(),
        Greater => {
            let current_pos = seeker.stream_position()?;
            seek_start(seeker, current_pos + current as u64)
        }
    }
}

impl Read for MultiSeekReaders {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.buf[self.pos].data.read(buf)?;
        if size == buf.len() || self.pos >= self.buf.len() - 1 {
            Ok(size)
        } else {
            self.pos += 1;
            self.buf[self.pos].data.seek(SeekFrom::Start(0))?;
            Ok(size + self.read(&mut buf[size..])?)
        }
    }
}
