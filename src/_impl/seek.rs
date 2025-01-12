use crate::MultiReaders;

use std::io::{Seek, SeekFrom};
impl<T: Seek> MultiReaders<T> {
    fn len(&mut self) -> std::io::Result<u64> {
        if self.len == 0 {
            for r in &mut self.inner {
                self.len += r.len()?;
            }
        }
        Ok(self.len)
    }

    fn position(&mut self) -> std::io::Result<u64> {
        let mut pos = 0;
        for item in &mut self.inner[..self.pos] {
            pos += item.len()?;
        }
        pos += self.inner[self.pos].stream_position()?;
        Ok(pos)
    }
}

impl<T: Seek> Seek for MultiReaders<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        if self.inner.len() == self.pos {
            return Ok(0);
        }
        match pos {
            SeekFrom::Start(s) => seek_start(self, s),
            SeekFrom::End(e) => seek_end(self, e),
            SeekFrom::Current(c) => seek_current(self, c),
        }
    }
}

fn seek_start<T: Seek>(r: &mut MultiReaders<T>, start: u64) -> std::io::Result<u64> {
    let mut len = 0;
    for i in 0..r.inner.len() {
        let tmp = len;
        len += r.inner[i].len()?;
        if len >= start {
            r.pos = i;
            r.inner[i].seek(SeekFrom::Start(start - tmp))?;
            for j in r.pos + 1..r.inner.len() {
                r.inner[j].seek(SeekFrom::Start(0))?;
            }
            return Ok(start);
        } else {
            r.inner[i].seek(SeekFrom::End(0))?;
        }
    }
    Ok(len)
}

fn seek_to_end<T: Seek>(r: &mut MultiReaders<T>) -> std::io::Result<u64> {
    for item in &mut r.inner {
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
