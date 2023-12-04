use crate::SeekRead;
use std::{
    cmp::Ordering,
    fmt::Debug,
    io::{Read, Result, Seek, SeekFrom},
};

struct BoxReader<R> {
    r: R,
    len: u64,
}
impl<R: Debug> Debug for BoxReader<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxReader")
            .field("r", &self.r)
            .field("len", &self.len)
            .finish()
    }
}
impl<R: SeekRead> BoxReader<R> {
    pub fn new(mut r: R) -> Result<Self> {
        let len = r.seek(SeekFrom::End(0))?;
        r.rewind()?;
        Ok(Self { r, len })
    }
}
/// Merge multiple streams into one stream
#[derive(Default)]
pub struct MultiReaders<R> {
    buf: Vec<BoxReader<R>>,
    len: u64,
    pos: u64,
    index: usize,
}
impl<R: Debug> Debug for MultiReaders<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiReaders")
            .field("buf", &self.buf)
            .field("len", &self.len)
            .field("pos", &self.pos)
            .field("index", &self.index)
            .finish()
    }
}
impl<R: SeekRead> MultiReaders<R> {
    /// Create a new empty `MultiReaders`
    pub fn new() -> MultiReaders<R> {
        Self {
            buf: Vec::new(),
            len: 0,
            pos: 0,
            index: 0,
        }
    }
    /// Appends an element.
    ///
    /// # Error
    ///
    /// To get the length of `r`, method `r.seek(SeekFrom::End(0))` will be called.
    ///
    /// Seeking can fail, for example because it might involve flushing a buffer.
    pub fn push(&mut self, r: R) -> Result<()> {
        let r = BoxReader::new(r)?;
        let len = r.len;
        if len > 0 {
            self.buf.push(r);
            self.len += len;
        }
        Ok(())
    }
    /// Return `true` if the stream is empty
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Return `true` if the stream is eof
    pub fn is_end(&self) -> bool {
        self.pos == self.len
    }
    /// Return the length of the stream
    pub fn len(&self) -> u64 {
        self.len
    }
    /// Return the current position
    pub fn pos(&self) -> u64 {
        let mut pos = self.pos;
        for r in &self.buf[..self.index] {
            pos += r.len;
        }
        pos
    }

    fn add_offset(&mut self, offset: u64) -> Result<u64> {
        if self.len > offset + self.pos() {
            let remain = self.buf[self.index].len - self.pos - 1;
            if remain >= offset {
                self.pos = self.buf[self.index]
                    .r
                    .seek(SeekFrom::Current(offset as i64))?;
            } else {
                self.index += 1;
                self.pos = offset - remain - 1;
                while self.pos > self.buf[self.index].len {
                    self.pos -= self.buf[self.index].len;
                    self.index += 1;
                }
                self.buf[self.index].r.seek(SeekFrom::Start(self.pos))?;
            }
            Ok(self.pos())
        } else {
            self.seek_end()?;
            Ok(if self.is_empty() { 0 } else { self.len - 1 })
        }
    }
    fn sub_offset(&mut self, offset: u64) -> Result<u64> {
        if self.pos() >= offset {
            if self.pos >= offset {
                self.pos = self.buf[self.index]
                    .r
                    .seek(SeekFrom::Current(-(offset as i64)))?;
            } else {
                self.index -= 1;
                let mut n = offset as i64 - self.pos as i64 - 1;
                while n < 0 {
                    n += self.buf[self.index].len as i64;
                    self.index -= 1;
                }
                self.buf[self.index].r.seek(SeekFrom::End(n.abs()))?;
            }
            Ok(self.pos())
        } else {
            self.seek_start()?;
            Ok(0)
        }
    }

    fn seek_start(&mut self) -> Result<()> {
        self.index = 0;
        self.pos = 0;
        for r in &mut self.buf {
            r.r.rewind()?;
        }
        Ok(())
    }
    fn seek_end(&mut self) -> Result<()> {
        if self.buf.is_empty() {
            return Ok(());
        }
        for r in &mut self.buf {
            r.r.rewind()?;
        }
        self.index = self.buf.len() - 1;
        self.pos = self.buf[self.index].r.seek(SeekFrom::End(0))?;
        Ok(())
    }
}

impl<R: SeekRead> Read for MultiReaders<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.pos() >= self.len {
            return Ok(0);
        }
        let len = self.buf[self.index].r.read(buf)?;
        self.pos += len as u64;
        if len < buf.len() {
            self.index += 1;
            self.pos = 0;
            Ok(self.read(&mut buf[len..])? + len)
        } else {
            if self.pos >= self.buf[self.index].len {
                self.index += 1;
                self.pos = 0;
            }
            Ok(len)
        }
    }
}

impl<R: SeekRead> Seek for MultiReaders<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Current(i) => match i.cmp(&0) {
                Ordering::Equal => Ok(self.pos()),
                Ordering::Greater => self.add_offset(i.unsigned_abs()),
                Ordering::Less => self.sub_offset(i.unsigned_abs()),
            },
            SeekFrom::End(end) => {
                if end >= 0 {
                    self.seek_end()?;
                    Ok(self.len)
                } else {
                    self.seek_end()?;
                    self.sub_offset(end.unsigned_abs())
                }
            }
            SeekFrom::Start(start) => {
                self.seek_start()?;
                self.add_offset(start)
            }
        }
    }
}
