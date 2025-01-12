#[macro_export]
macro_rules! wrap {
    (dyn $trait:ident, $($val:expr), +) => {
        $crate::MultiReaders::new( [ $(Box::new($val) as Box<dyn $trait>), + ].into_iter())
    };
    ($($val:expr), +) => {
        $crate::MultiReaders::new( [ $($val), + ].into_iter())
    };
}

#[macro_export]
macro_rules! open {
    (async $construct:expr, [$($path:expr), +]) => {
        $crate::wrap!($($construct($path).await), +)
    };
    ($construct:expr, [$($path:expr), +]) => {
        $crate::wrap!($($construct($path)), +)
    };
}
