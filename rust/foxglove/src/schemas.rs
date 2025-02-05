//! Types implementing well-known Foxglove schemas
//!
//! Using these types when possible will allow for richer visualizations
//! and a better experience in the Foxglove App.
//!
//! They're encoded as compact, binary protobuf messages,
//! and can be conveniently used with the [`TypedChannel`](crate::TypedChannel) API.

pub(crate) mod descriptors;
#[allow(missing_docs)]
#[rustfmt::skip]
mod foxglove;
#[rustfmt::skip]
mod impls;

pub use self::foxglove::*;
