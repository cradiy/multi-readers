#[macro_export]
macro_rules! wrap {
    (dyn $trait:ident, $($val:expr), +) => {
        $crate::MultiReaders::new( vec![ $(Box::new($val) as Box<dyn $trait>), + ])
    };
    ($($val:expr), +) => {
        $crate::MultiReaders::new( vec![ $($val), + ])
    };
}
#[macro_export]
macro_rules! open {
    ($($path:expr), +) => {
        $crate::wrap!($($crate::lazy_file::LazyFile::new($path)) , +)
    };
    (dyn $trait:ident, $($path:expr), +) => {
        $crate::wrap!($($crate::lazy_file::LazyFile::new($path)) , +)
    };
}

#[deprecated = "Use `wrap!` instead"]
/// wrap multiple readers into a single
#[macro_export]
macro_rules! join_readers {
    ($($r:expr), +) => {
        $crate::wrap!(dyn ::std::io::Read, $($r), +)
    };
}

#[cfg(feature = "async")]
/// wrap multiple async readers into a single
#[macro_export]
macro_rules! join_async_readers {
    ($($r:expr), +) => {
        $crate::AsyncMultiReaders::from_iter(vec![$(Box::new($r) as Box<dyn tokio::io::AsyncRead + Unpin>, )+].into_iter())
    };
}
