use std::io::{Read, Seek};
pub(crate) struct Inner<T> {
    buf: T,
    len: Option<u64>,
}
impl<T> Inner<T> {
    pub(crate) fn new(buf: T) -> Self {
        Self { buf, len: None }
    }
    pub(crate) fn inner(self) -> T {
        self.buf
    }
    #[allow(dead_code)]
    pub fn get(&self) -> &T {
        &self.buf
    }
}

impl<T: Read> Read for Inner<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buf.read(buf)
    }
}

impl<T: Seek> Seek for Inner<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.buf.seek(pos)
    }
}

impl<T: Seek> Inner<T> {
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
#[cfg(feature = "async")]
impl<T: tokio::io::AsyncRead + Unpin> tokio::io::AsyncRead for Inner<T> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        tokio::io::AsyncRead::poll_read(std::pin::pin!(&mut self.get_mut().buf), cx, buf)
    }
}
