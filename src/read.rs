use std::io::Read;


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
