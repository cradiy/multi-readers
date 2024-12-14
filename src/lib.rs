mod _impl;
pub mod lazy_file;
pub use _impl::MultiReaders;
#[cfg(feature = "async")]
mod async_impl;
mod wrap;
#[cfg(feature = "async")]
pub use async_impl::AsyncMultiReaders;
