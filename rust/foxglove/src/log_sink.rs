use crate::channel::Channel;
use crate::metadata::Metadata;
use crate::FoxgloveError;
use std::sync::Arc;

/// A `LogSink` writes log messages to their destination.
///
/// LogSinks are thread-safe and can be shared between threads.
/// Usually you'd use our implementations like [`McapWriter`](crate::McapWriter) or [`WebSocketServer`](crate::WebSocketServer).
pub trait LogSink: Send + Sync {
    /// log writes the message for the channel to the sink.
    /// metadata contains optional message metadata that may be used by some sink implementations.
    fn log(
        &self,
        channel: &Arc<Channel>,
        msg: &[u8],
        metadata: &Metadata,
    ) -> Result<(), FoxgloveError>;

    /// add_channel is called when a new channel is associated with this Sink.
    /// Sinks can track channels seen, and do new channel related things the first
    /// time they see a channel, rather than in this method. The choice is up to the implementor.
    fn add_channel(&self, _channel: &Arc<Channel>) {}

    /// remove_channel is called when a channel is unassociated with this Sink.
    /// Sinks can clean up any channel-related state they have or take other actions.
    fn remove_channel(&self, _channel: &Channel) {}
}
