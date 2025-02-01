use thiserror::Error;

mod channel;
mod channel_builder;
mod collection;
mod cow_vec;
mod log_context;
mod log_sink;
mod log_sink_set;
#[cfg(test)]
mod log_sink_test;
mod mcap_writer;
mod metadata;
pub mod schemas;
mod time;
mod typed_channel;
pub mod websocket;
mod websocket_server;

pub use channel::{Channel, Schema};
pub use channel_builder::ChannelBuilder;
pub use log_context::{GlobalContextTest, LogContext};
pub use log_sink::LogSink;
pub use mcap_writer::{McapWriter, McapWriterHandle};
pub use metadata::{Metadata, PartialMetadata};
pub use time::nanoseconds_since_epoch;
pub use typed_channel::{TypedChannel, TypedMessage};
pub use websocket_server::{WebSocketServer, WebSocketServerHandle};

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
