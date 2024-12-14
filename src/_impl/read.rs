use std::io::{Read, Seek, SeekFrom};

pub(crate) struct Buf<T> {
    buf: T,
    len: Option<u64>,
}
impl<T> Buf<T> {
    pub(crate) fn new(buf: T) -> Self {
        Self { buf, len: None }
    }
}

impl<T: Read> Read for Buf<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buf.read(buf)
    }
}

impl<T: Seek> Seek for Buf<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.buf.seek(pos)
    }
}

impl<T: Seek> Buf<T> {
    pub(crate) fn len(&mut self) -> std::io::Result<u64> {
        if let Some(len) = self.len {
            Ok(len)
        } else {
            let pos = self.buf.stream_position()?;
            let len = self.buf.seek(std::io::SeekFrom::End(0))?;
            self.buf.seek(std::io::SeekFrom::Start(pos))?;
            self.len = Some(len);
            Ok(len)
        }
    }
}

/// Wrapper for multiple readers
pub struct MultiReaders<T> {
    pub(crate) buf: Vec<Buf<T>>,
    pub(crate) len: u64,
    pub(crate) pos: usize,
}

impl<T> MultiReaders<T> {
    pub fn new(values: Vec<T>) -> Self {
        if values.is_empty() {
            panic!("MultiReaders cannot be empty!")
        }
        Self {
            buf: values.into_iter().map(|v| Buf::new(v)).collect(),
            pos: 0,
            len: 0,
        }
    }
    pub fn try_new(iter: impl Iterator<Item = T> + 'static) -> Option<Self> {
        let buf: Vec<_> = iter.map(|t| Buf::new(t)).collect();
        if buf.is_empty() {
            None
        } else {
            Some(Self {
                buf,
                pos: 0,
                len: 0,
            })
        }
    }
}

impl<T: Read> Read for MultiReaders<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pos < self.buf.len() {
            let len = self.buf[self.pos].read(buf)?;
            if len < buf.len() {
                self.pos += 1;
                Ok(len + self.read(&mut buf[len..])?)
            } else {
                Ok(len)
            }
        } else {
            Ok(0)
        }
    }
}

impl<T: Seek> MultiReaders<T> {
    fn len(&mut self) -> std::io::Result<u64> {
        if self.len == 0 {
            for r in &mut self.buf {
                self.len += r.len()?;
            }
        }
        Ok(self.len)
    }

    fn position(&mut self) -> std::io::Result<u64> {
        let mut pos = 0;
        for item in &mut self.buf[..self.pos] {
            pos += item.len()?;
        }
        pos += self.buf[self.pos].stream_position()?;
        Ok(pos)
    }
}

impl<T: Seek> Seek for MultiReaders<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match pos {
            std::io::SeekFrom::Start(s) => seek_start(self, s),
            std::io::SeekFrom::End(e) => seek_end(self, e),
            std::io::SeekFrom::Current(c) => seek_current(self, c),
        }
    }
}

fn seek_start<T: Seek>(r: &mut MultiReaders<T>, start: u64) -> std::io::Result<u64> {
    let mut len = 0;
    for i in 0..r.buf.len() {
        let tmp = len;
        len += r.buf[i].len()?;
        if len >= start {
            r.pos = i;
            r.buf[i].seek(SeekFrom::Start(start - tmp))?;
            for j in r.pos + 1..r.buf.len() {
                r.buf[j].seek(SeekFrom::Start(0))?;
            }
            return Ok(start);
        } else {
            r.buf[i].seek(SeekFrom::End(0))?;
        }
    }
    Ok(len)
}

fn seek_to_end<T: Seek>(r: &mut MultiReaders<T>) -> std::io::Result<u64> {
    for item in &mut r.buf {
        item.seek(SeekFrom::End(0))?;
    }
    r.len()
}

fn seek_end<T: Seek>(r: &mut MultiReaders<T>, end: i64) -> std::io::Result<u64> {
    if end >= 0 {
        seek_to_end(r)
    } else {
        let len = r.len()? as i64;
        seek_start(r, (len + end) as _)
    }
}

fn seek_current<T: Seek>(r: &mut MultiReaders<T>, current: i64) -> std::io::Result<u64> {
    if current == 0 {
        r.position()
    } else {
        let n = r.position()? as i64 + current;
        let len = r.len()? as i64;
        let n = n.max(0).min(len);
        seek_start(r, n as u64)
    }
}
