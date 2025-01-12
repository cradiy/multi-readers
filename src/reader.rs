use crate::inner::Inner;

/// Wrapper for multiple readers
pub struct MultiReaders<T> {
    pub(crate) inner: Vec<Inner<T>>,
    pub(crate) len: u64,
    pub(crate) pos: usize,
    #[cfg(feature = "async")]
    pub(crate) buf: Vec<u8>,
    #[cfg(feature = "async")]
    pub(crate) filled: usize,
}

impl<T> MultiReaders<T> {
    pub fn new<I: Iterator<Item = T> + 'static>(values: I) -> Self {
        Self {
            inner: values.map(Inner::new).collect(),
            pos: 0,
            len: 0,
            #[cfg(feature = "async")]
            buf: Vec::new(),
            #[cfg(feature = "async")]
            filled: 0,
        }
    }
}

impl<T: IntoIterator<Item = E>, E> MultiReaders<T> {
    /// Creates an iterator that flattens nested structure and collect them into a new `MultiReaders`.
    ///
    /// # Basic Usage
    ///
    /// ```rust
    /// use std::fs::File;
    /// use std::io::Result;
    /// use multi_readers::{open, MultiReaders};
    ///
    /// let r1: MultiReaders<Result<File>> = open!(File::open, ["path1", "path2"]);
    /// let mut _readers: MultiReaders<File> = r1.flatten();
    /// ```
    /// # Panics
    /// Panics if called after the first read
    pub fn flatten(self) -> MultiReaders<E> {
        if self.pos != 0 {
            panic!("Only allowed to be called during initialization!")
        }
        let inner = self
            .inner
            .into_iter()
            .flat_map(Inner::inner)
            .map(Inner::new)
            .collect();
        MultiReaders {
            inner,
            pos: 0,
            len: 0,
            #[cfg(feature = "async")]
            buf: Vec::new(),
            #[cfg(feature = "async")]
            filled: 0,
        }
    }
}
