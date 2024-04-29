use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};
use crate::{BytesReader, SliceReader};
use crate::reader::HandleFunc;

impl AsyncRead for BytesReader {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut slice_reader = self.as_slice_reader();
        let result = tokio::io::AsyncRead::poll_read(Pin::new(&mut slice_reader), _cx, buf);
        self.get_mut().pos = slice_reader.pos;
        result
    }
}
impl<'a> AsyncRead for SliceReader<'a> {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if self.pos == self.buf.len() {
            return Poll::Ready(Ok(()));
        }
        let amt = std::cmp::min(self.buf.len() - self.pos, buf.remaining());
        buf.put_slice(&self.buf[self.pos..self.pos + amt]);
        self.get_mut().pos += amt;
        Poll::Ready(Ok(()))
    }
}

pub struct AsyncMultiReaders<'iter, 'life, 'func> {
    current: Option<Box<dyn AsyncRead + Unpin + 'life>>,
    iter: Box<dyn Iterator<Item = Box<dyn AsyncRead + Unpin + 'life>> + 'iter>,
    #[allow(dead_code)]
    /// TODO
    func: HandleFunc<'func>
}

impl<'iter, 'life, 'func> AsyncMultiReaders<'iter, 'life, 'func> {
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter(iter: impl Iterator<Item = Box<dyn AsyncRead + Unpin + 'life>> + 'iter) -> Self {
        Self {
            current: None,
            iter: Box::new(iter),
            func: None
        }
    }
}

impl<'iter, 'life, 'func> AsyncRead for AsyncMultiReaders<'iter, 'life, 'func> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        if buf.remaining() > 0 {
            if this.current.is_none() {
                this.current = this.iter.next();
            }
            match &mut this.current {
                Some(r) => {
                    match tokio::io::AsyncRead::poll_read(Pin::new(r), cx, buf)? { 
                        Poll::Ready(_) => {
                            if buf.remaining() > 0 {
                                this.current = this.iter.next();
                                AsyncRead::poll_read(Pin::new(this), cx, buf)
                            } else {
                                Poll::Ready(Ok(()))
                            }
                        }
                        Poll::Pending => Poll::Pending
                    }
                }
                None => Poll::Ready(Ok(()))
            }
        } else {
            Poll::Ready(Ok(()))
        }
    }
}