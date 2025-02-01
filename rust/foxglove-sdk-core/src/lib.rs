use thiserror::Error;

mod channel;
mod collection;
mod cow_vec;
mod log_context;
pub mod log_file;
mod log_sink;
mod log_sink_set;
#[cfg(test)]
mod log_sink_test;
mod metadata;
mod time;
pub mod websocket;

pub use channel::{Channel, Schema};
pub use log_context::LogContext;
pub use log_file::FileWriter;
pub use log_sink::LogSink;
pub use metadata::{Metadata, PartialMetadata};
pub use time::nanoseconds_since_epoch;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FoxgloveError {
    #[error("Fatal error: {0}")]
    Fatal(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("MCAP error: {0}")]
    MCAPError(#[from] mcap::McapError),
    #[allow(dead_code)]
    #[error(transparent)]
    JSONError(#[from] serde_json::Error),
}
