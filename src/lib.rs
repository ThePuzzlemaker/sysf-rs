pub mod ast;
pub mod ctx;
pub mod grammar;
pub mod pp;
pub mod typeck;

#[macro_export]
macro_rules! trace {
    ($($tt:tt)*) => {
        #[cfg(feature = "trace")]
        {
            ::tracing::trace!($($tt)*)
        }
    };
}
