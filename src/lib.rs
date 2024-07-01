mod read;
mod reader;
#[cfg(feature = "async")]
mod asynchronous;
#[cfg(feature = "async")]
pub use asynchronous::AsyncMultiReaders;
pub use reader::{BytesReader, MultiReaders, SliceReader};
pub use read::read;
/// Join multiple readers into a single
///
/// # Example
/// ```no_run
/// use multi_readers::{join_readers, BytesReader};
/// use std::fs::File;
///
/// let f = File::open("path").unwrap();
/// let bytes = vec![1, 2, 3, 4];
/// let br = BytesReader::new(bytes);
///
/// let mut reader = join_readers!(f, br);
/// ```
#[macro_export]
macro_rules! join_readers {
    ($($r:expr), +) => {
        $crate::MultiReaders::from_iter(vec![$(Box::new($r) as Box<dyn std::io::Read>, )+].into_iter())
    };
}

#[macro_export]
macro_rules! join_paths_to_readers {
    ($($path:expr), +) => {
        {
            fn __open_file__() -> std::io::Result<Vec<Box<dyn std::io::Read>>> {
                Ok(vec![$(Box::new(std::fs::File::open($path)?) as Box<dyn std::io::Read>, )+])
            }
            __open_file__().map(|readers| {
                $crate::MultiReaders::from_iter(readers.into_iter())
            })
        }
    };
}

#[cfg(feature = "async")]
/// Join multiple async readers into a single
#[macro_export]
macro_rules! join_async_readers {
    ($($r:expr), +) => {
        $crate::AsyncMultiReaders::from_iter(vec![$(Box::new($r) as Box<dyn tokio::io::AsyncRead + Unpin>, )+].into_iter())
    };
}
