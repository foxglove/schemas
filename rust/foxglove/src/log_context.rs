use crate::log_sink_set::LogSinkSet;
use crate::{Channel, FoxgloveError, LogSink};
use parking_lot::{Mutex, MutexGuard, RwLock};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

/// A wrapper to store a Channel in a HashSet by topic.
#[derive(Clone)]
pub struct ChannelByTopic(Arc<Channel>);

impl PartialEq for ChannelByTopic {
    fn eq(&self, other: &Self) -> bool {
        self.0.topic == other.topic
    }
}

impl Eq for ChannelByTopic {}

impl Hash for ChannelByTopic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.topic.hash(state);
    }
}

impl Borrow<str> for ChannelByTopic {
    fn borrow(&self) -> &str {
        self.0.topic.as_str()
    }
}

impl Deref for ChannelByTopic {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A thread-safe wrapper around one or more Sinks, that writes to all of them.
pub struct LogContext {
    // Map of channels by topic.
    channels: RwLock<HashSet<ChannelByTopic>>,
    sinks: LogSinkSet,
}

impl LogContext {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashSet::new()),
            sinks: LogSinkSet::new(),
        }
    }

    pub fn global() -> &'static LogContext {
        static DEFAULT_CONTEXT: OnceLock<LogContext> = OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(LogContext::new)
    }

    pub fn get_channel_by_topic(&self, topic: &str) -> Option<Arc<Channel>> {
        let channels = self.channels.read();
        channels.get(topic).map(|channel| channel.0.clone())
    }

    pub fn add_channel(&self, channel: Arc<Channel>) -> Result<(), FoxgloveError> {
        {
            // Wrapped in a block, so we release the lock immediately.
            let mut channels = self.channels.write();
            if !channels.insert(ChannelByTopic(channel.clone())) {
                return Err(FoxgloveError::Fatal(format!(
                    "Channel id={} for topic {} already exists in registry",
                    channel.id, channel.topic,
                )));
            }
        }
        self.sinks.for_each(|sink| {
            if channel.sinks.add_sink(sink.clone()) {
                sink.add_channel(&channel);
            }
            Ok(())
        });
        Ok(())
    }

    pub fn remove_channel_for_topic(&self, topic: &str) -> bool {
        let maybe_channel_by_topic = {
            let mut channels = self.channels.write();
            channels.take(topic)
        };

        let Some(channel_by_topic) = maybe_channel_by_topic else {
            // Channel not found.
            return false;
        };
        let channel = &*channel_by_topic.0;

        self.sinks.for_each(|sink| {
            if channel.sinks.remove_sink(sink) {
                sink.remove_channel(channel);
            }
            Ok(())
        });
        true
    }

    pub fn add_sink(&self, sink: Arc<dyn LogSink>) -> bool {
        if !self.sinks.add_sink(sink.clone()) {
            return false;
        }

        // Add the sink to all existing channels.
        for channel in self.channels.read().iter() {
            if channel.sinks.add_sink(sink.clone()) {
                sink.add_channel(&channel.0);
            }
        }

        true
    }

    pub fn remove_sink(&self, sink: &Arc<dyn LogSink>) -> bool {
        if !self.sinks.remove_sink(sink) {
            return false;
        }

        // TODO this has a bug, if the same sink was added to a channel twice, via two different LogContexts,
        // this will remove the sink from the channel, even although they're still associated via the other LogContext.
        // If we stored the contexts on the channel, and removed the contexts, it would fix it,
        // But logging would be via an extra indirection to LogContext (slower) and
        // having it associated with the same sink twice would result in two log calls to the sink,
        // which is a more serious bug.
        // I think the solution should be to have both the contexts and the sinks on the channel.
        // This also fixes the problems with Channel::close().
        // FG-9893

        // Remove the sink from all existing channels.
        for channel in self.channels.read().iter() {
            if channel.sinks.remove_sink(sink) {
                sink.remove_channel(&channel.0);
            }
        }

        true
    }

    pub fn clear(&self) {
        // This seems like a false positive, ChannelByTopic is not mutable.
        #[allow(clippy::mutable_key_type)]
        let channels: HashSet<_> = { std::mem::take(&mut self.channels.write()) };
        self.sinks.for_each(|sink| {
            for channel in channels.iter() {
                sink.remove_channel(&channel.0);
                channel.sinks.clear();
            }
            Ok(())
        });
        self.sinks.clear();
    }
}

impl Drop for LogContext {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Default for LogContext {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_CONTEXT_TEST_LOCK: Mutex<()> = Mutex::new(());

/// A helper to synchronize tests that use the global context, and clear it afterwards.
#[doc(hidden)]
pub struct GlobalContextTest<'a>(#[allow(dead_code)] MutexGuard<'a, ()>);

impl GlobalContextTest<'_> {
    pub fn new() -> Self {
        Self(GLOBAL_CONTEXT_TEST_LOCK.lock())
    }
}

impl Drop for GlobalContextTest<'_> {
    fn drop(&mut self) {
        LogContext::global().clear();
    }
}

impl Default for GlobalContextTest<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::channel::ChannelId;
    use crate::collection::collection;
    use crate::log_context::*;
    use crate::log_sink_set::ERROR_LOGGING_MESSAGE;
    use crate::log_sink_test::{ErrorSink, MockSink, RecordingSink};
    use crate::{nanoseconds_since_epoch, Channel, PartialMetadata, Schema};
    use std::sync::atomic::AtomicU32;
    use std::sync::Arc;
    use tracing_test::traced_test;

    fn new_test_channel(id: u64) -> Arc<Channel> {
        Arc::new(Channel {
            sinks: LogSinkSet::new(),
            id: ChannelId::new(id),
            message_sequence: AtomicU32::new(1),
            topic: "topic".to_string(),
            message_encoding: "message_encoding".to_string(),
            schema: Some(Schema::new(
                "name",
                "encoding",
                br#"{
                    "type": "object",
                    "properties": {
                        "msg": {"type": "string"},
                        "count": {"type": "number"},
                    },
                }"#,
            )),
            metadata: collection! {"key".to_string() => "value".to_string()},
        })
    }

    #[test]
    fn test_add_and_remove_sink() {
        let ctx = LogContext::new();
        let sink = Arc::new(MockSink);
        let sink2 = Arc::new(MockSink);
        let sink3 = Arc::new(MockSink);

        // Test adding a sink
        assert!(ctx.add_sink(sink.clone()));
        // Can't add it twice
        assert!(!ctx.add_sink(sink.clone()));
        assert!(ctx.add_sink(sink2.clone()));

        // Test removing a sink
        let sink: Arc<dyn LogSink> = sink;
        assert!(ctx.remove_sink(&sink));

        // Try to remove a sink that doesn't exist
        let sink3: Arc<dyn LogSink> = sink3;
        assert!(!ctx.remove_sink(&sink3));

        // Test removing the last sink
        let sink2: Arc<dyn LogSink> = sink2;
        assert!(ctx.remove_sink(&sink2));
    }

    #[traced_test]
    #[test]
    fn test_log_calls_sinks() {
        let ctx = LogContext::new();
        let sink1 = Arc::new(RecordingSink::new());
        let sink2 = Arc::new(RecordingSink::new());

        assert!(ctx.add_sink(sink1.clone()));
        assert!(ctx.add_sink(sink2.clone()));

        let channel = new_test_channel(1);
        ctx.add_channel(channel.clone()).unwrap();
        let msg = b"test_message";

        let now = nanoseconds_since_epoch();

        channel.log(msg);
        assert!(!logs_contain(ERROR_LOGGING_MESSAGE));

        let recorded1 = sink1.recorded.lock();
        let recorded2 = sink2.recorded.lock();

        assert_eq!(recorded1.len(), 1);
        assert_eq!(recorded2.len(), 1);

        assert_eq!(&recorded1[0].channel, &channel);
        assert_eq!(recorded1[0].msg, msg.to_vec());
        let metadata1 = &recorded1[0].metadata;
        assert!(metadata1.log_time >= now);
        assert!(metadata1.publish_time >= now);
        assert_eq!(metadata1.log_time, metadata1.publish_time);
        assert!(metadata1.sequence > 0);

        assert_eq!(&recorded2[0].channel, &channel);
        assert_eq!(recorded2[0].msg, msg.to_vec());
        let metadata2 = &recorded2[0].metadata;
        assert!(metadata2.log_time >= now);
        assert!(metadata2.publish_time >= now);
        assert_eq!(metadata2.log_time, metadata2.publish_time);
        assert!(metadata2.sequence > 0);
        assert_eq!(metadata1.sequence, metadata2.sequence);
    }

    #[traced_test]
    #[test]
    fn test_log_calls_other_sinks_after_error() {
        let ctx = LogContext::new();
        let error_sink = Arc::new(ErrorSink);
        let recording_sink = Arc::new(RecordingSink::new());

        assert!(ctx.add_sink(error_sink.clone()));
        assert!(!ctx.add_sink(error_sink.clone()));
        assert!(ctx.add_sink(recording_sink.clone()));

        let channel = new_test_channel(1);
        ctx.add_channel(channel.clone()).unwrap();
        let msg = b"test_message";
        let opts = PartialMetadata {
            sequence: Some(1),
            log_time: Some(nanoseconds_since_epoch()),
            publish_time: Some(nanoseconds_since_epoch()),
        };

        channel.log_with_meta(msg, opts);
        assert!(logs_contain(ERROR_LOGGING_MESSAGE));
        assert!(logs_contain("ErrorSink always fails"));

        let recorded = recording_sink.recorded.lock();
        assert_eq!(recorded.len(), 1);
        assert_eq!(&recorded[0].channel, &channel);
        assert_eq!(recorded[0].msg, msg.to_vec());
        let metadata = &recorded[0].metadata;
        assert_eq!(metadata.sequence, opts.sequence.unwrap());
        assert_eq!(metadata.log_time, opts.log_time.unwrap());
        assert_eq!(metadata.publish_time, opts.publish_time.unwrap());
    }

    #[traced_test]
    #[test]
    fn test_log_msg_no_sinks() {
        let channel = Arc::new(new_test_channel(1));
        let msg = b"test_message";

        channel.log(msg);
        assert!(!logs_contain(ERROR_LOGGING_MESSAGE));
    }
}
