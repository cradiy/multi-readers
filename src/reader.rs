use crate::inner::Inner;

/// A structure that manages multiple readers, allowing sequential or concurrent
/// access to a collection of inner readers.
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
    /// Creates a new `MultiReaders` instance from an iterator of values.
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
    /// Creates a new `MultiReaders` instance by flattening the nested structure of the current instance.
    ///
    /// This method transforms the nested structure of the `MultiReaders` into a single-level structure
    /// by applying a `flat_map` operation on the inner elements. 
    /// 
    /// # Panics
    ///
    /// This method will panic if it is called after the first read operation.
    /// It is intended to be used only during the initialization phase of the `MultiReaders`.
    ///
    /// # Examples
    /// 
    /// ```rust
    /// 
    /// use std::io::{Cursor, Read};
    /// use multi_readers::{open, MultiReaders};
    /// 
    /// let hello = "hello";
    /// let world = "world";
    /// let none = "none";
    /// let my_open = |s: &'static str | {
    ///     if s == "none" {
    ///        None
    ///    } else {
    ///       Some(Cursor::new(s))
    ///   }
    /// };
    /// let readers: MultiReaders<Option<Cursor<&str>>> = open!(my_open, [hello, none, world]);
    /// let mut readers: MultiReaders<Cursor<&str>> = readers.flatten();
    /// let mut buf = String::new();
    /// readers.read_to_string(&mut buf).unwrap();
    /// assert_eq!(&buf, "helloworld");
    /// ```
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
