/// collection! is like vec! but for maps and sets (and other collections).
///
/// Map-like: Takes a series of key-value pairs separated by => and optionally terminated by a comma.
/// It uses the Iterator::collect method to transform an array of tuples into a collection.
/// The type of collection will be inferred by the Rust compiler based on the context in which the macro is used.
/// For example, if the surrounding code expects a HashMap, the compiler will attempt to collect the tuples into a HashMap.
///
/// Set-like: Takes a series of values separated by commas, again optionally terminated by a comma.
/// Just like the map-like pattern, it uses Iterator::collect to transform an array of values into a collection,
/// which could be a HashSet or another set-like collection depending on the type expected in the context.
// Source: https://stackoverflow.com/a/27582993/152580
#[doc(hidden)]
#[macro_export]
macro_rules! collection {
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

#[allow(unused_imports)]
pub(crate) use collection;
