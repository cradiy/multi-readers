use crate::MultiReaders;

use std::io::Read;

impl<T: Read> Read for MultiReaders<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.inner.len() == self.pos {
            return Ok(0);
        }
        let len = self.inner[self.pos].read(buf)?;
        if len < buf.len() {
            self.pos += 1;
            Ok(len + self.read(&mut buf[len..])?)
        } else {
            Ok(len)
        }
    }
}
