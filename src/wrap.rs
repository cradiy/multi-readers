#[macro_export]
/// A macro to create a `MultiReaders` instance from a list of values.
/// 
/// # Examples
/// 
/// ```rust,ignore
/// // Using the macro with trait objects
/// wrap!(dyn MyTrait, val1, val2, val3);
/// 
/// // Using the macro with regular values
/// wrap!(val1, val2, val3);
/// ```
/// 
/// # Parameters
/// 
/// - `dyn $trait`: The trait that the values implement (optional).
/// - `$val`: The values to be wrapped in the `MultiReaders` instance.
/// 
/// # Usage
/// 
/// - When using the `dyn $trait` form, each value will be boxed and cast to a trait object.
/// - When using the form without `dyn $trait`, the values will be used as they are.
macro_rules! wrap {
    (dyn $trait:path, $($val:expr), +) => {
        $crate::MultiReaders::new( [ $(Box::new($val) as Box<dyn $trait>), + ].into_iter())
    };
    ($($val:expr), +) => {
        $crate::MultiReaders::new( [ $($val), + ].into_iter())
    };
}

#[macro_export]
/// A macro to create a `MultiReaders` instance by opening multiple values asynchronously or synchronously.
/// 
/// # Examples
/// 
/// ```rust,ignore
/// // Using the macro with asynchronous opening
/// open!(async my_async_open, [val1, val2, val3]);
/// 
/// // Using the macro with synchronous opening
/// open!(my_open, [val1, val2, val3]);
/// ```
/// 
/// # Parameters
/// 
/// - `async $open`: The asynchronous open function (optional).
/// - `$open`: The synchronous open function.
/// - `$arg`: The arguments to be passed to the open function.
/// 
/// # Usage
/// 
/// - When using the `async $open` form, each value will be awaited and then wrapped in the `MultiReaders` instance.
/// - When using the form without `async $open`, the values will be opened synchronously and wrapped in the `MultiReaders` instance.
macro_rules! open {
    (async $open:expr, [$($arg:expr), +]) => {
        $crate::wrap!($($open($arg).await), +)
    };
    ($open:expr, [$($arg:expr), +]) => {
        $crate::wrap!($($open($arg)), +)
    };
}
