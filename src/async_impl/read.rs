use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};

use crate::MultiReaders;

macro_rules! ready {
    ($n:expr) => {
        if ($n == 0) {
            return Poll::Ready(Ok(()));
        }
    };
    ($v1:expr, $v2:expr) => {
        if ($v1 == $v2) {
            return Poll::Ready(Ok(()));
        }
    };
}

impl<T: AsyncRead + Unpin> AsyncRead for MultiReaders<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        ready!(buf.remaining());
        if self.inner.len() == self.pos {
            return Poll::Ready(Ok(()));
        }
        let this = self.get_mut();
        if this.filled == 0 {
            this.buf = vec![0; buf.remaining()];
        }

        while this.filled < buf.remaining() {
            if let Some(val) = this.inner.get_mut(this.pos) {
                let mut tmp = ReadBuf::new(&mut this.buf[this.filled..]);
                match Pin::new(val).poll_read(cx, &mut tmp) {
                    Poll::Ready(Ok(_)) => {
                        this.filled += tmp.filled().len();
                        // Read EOF
                        if buf.remaining() > this.filled {
                            this.pos += 1;
                        }
                    }
                    Poll::Ready(Err(e)) => {
                        buf.put_slice(&this.buf[..this.filled]);
                        this.buf.clear();
                        this.filled = 0;
                        return Poll::Ready(Err(e));
                    }
                    Poll::Pending => return Poll::Pending,
                }
            } else {
                break;
            }
        }
        buf.put_slice(&this.buf[..this.filled]);
        this.buf.clear();
        this.filled = 0;
        Poll::Ready(Ok(()))
    }
}
