#[forbid(missing_docs)]
mod bytes;
#[forbid(missing_docs)]
mod multi;

use std::io::{Read, Seek};

pub use bytes::BytesReader;
pub use multi::MultiReaders;
/// [`Seek`][std::io::Seek] + [`Read`][std::io::Read]
pub trait SeekRead: Seek + Read {}
impl<R: Seek + Read> SeekRead for R {}
