use crate::log_sink_set::LogSinkSet;
use crate::{nanoseconds_since_epoch, Metadata, PartialMetadata};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct ChannelId(u64);

impl ChannelId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl From<ChannelId> for u64 {
    fn from(id: ChannelId) -> u64 {
        id.0
    }
}

impl std::fmt::Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Schema {
    pub name: String,
    pub encoding: String,
    pub data: Cow<'static, [u8]>,
}

impl Schema {
    pub fn new(
        name: impl Into<String>,
        encoding: impl Into<String>,
        data: impl Into<Cow<'static, [u8]>>,
    ) -> Self {
        Self {
            name: name.into(),
            encoding: encoding.into(),
            data: data.into(),
        }
    }
}

pub struct Channel {
    // TODO add public read-only accessors for these for the Rust API.
    // TODO add a list of contexts here as well (or restrict to one context per channel?)
    pub(crate) sinks: LogSinkSet,
    /// id is a unique identifier for the channel inside the process,
    /// it's used to map a channel to a channel_id or subscription_id in log sinks.
    pub(crate) id: ChannelId,
    pub(crate) message_sequence: AtomicU32,
    pub(crate) topic: String,
    pub(crate) message_encoding: String,
    pub(crate) schema: Option<Schema>,
    pub(crate) metadata: BTreeMap<String, String>,
}

impl Channel {
    pub fn id(&self) -> ChannelId {
        self.id
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn schema(&self) -> Option<&Schema> {
        self.schema.as_ref()
    }

    pub fn next_sequence(&self) -> u32 {
        self.message_sequence.fetch_add(1, Relaxed)
    }

    pub fn log(self: &Arc<Self>, msg: &[u8]) {
        self.log_with_meta(msg, PartialMetadata::new());
    }

    // Log a message to all sinks. Logs a warning for any errors.
    pub fn log_with_meta(self: &Arc<Self>, msg: &[u8], opts: PartialMetadata) {
        // Bail out early if there are no sinks (logging is disabled).
        if self.sinks.is_empty() {
            return;
        }

        let mut metadata = Metadata {
            sequence: opts.sequence.unwrap_or_else(|| self.next_sequence()),
            log_time: opts.log_time.unwrap_or_else(nanoseconds_since_epoch),
            publish_time: opts.publish_time.unwrap_or_default(),
        };
        // If publish_time is not set, use log_time.
        if opts.publish_time.is_none() {
            metadata.publish_time = metadata.log_time
        }

        self.sinks.for_each(|sink| sink.log(self, msg, &metadata));
    }
}

// For tests
impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.topic == other.topic
            && self.message_encoding == other.message_encoding
            && self.schema == other.schema
            && self.metadata == other.metadata
            && self.message_sequence.load(Relaxed) == other.message_sequence.load(Relaxed)
    }
}

impl Eq for Channel {}

impl std::fmt::Debug for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Channel")
            .field("id", &self.id)
            .field("topic", &self.topic)
            .field("message_encoding", &self.message_encoding)
            .field("schema", &self.schema)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.sinks.for_each(|sink| {
            sink.remove_channel(self);
            Ok(())
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::channel_builder::ChannelBuilder;
    use crate::collection::collection;
    use crate::log_context::{GlobalContextTest, LogContext};
    use crate::log_sink_set::ERROR_LOGGING_MESSAGE;
    use crate::log_sink_test::RecordingSink;
    use crate::Channel;
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
    fn test_channel_new() {
        let _cleanup = GlobalContextTest::new();
        let topic = "topic";
        let message_encoding = "message_encoding";
        let schema = Schema::new("schema_name", "schema_encoding", &[1, 2, 3]);
        let metadata: BTreeMap<String, String> =
            collection! {"key".to_string() => "value".to_string()};
        let channel = ChannelBuilder::new(topic)
            .message_encoding(message_encoding)
            .schema(schema.clone())
            .metadata(metadata.clone())
            .build()
            .expect("Failed to create channel");
        assert!(u64::from(channel.id) > 0);
        assert_eq!(channel.topic, topic);
        assert_eq!(channel.message_encoding, message_encoding);
        assert_eq!(channel.schema, Some(schema));
        assert_eq!(channel.metadata, metadata);
        assert_eq!(
            LogContext::global().get_channel_by_topic(topic),
            Some(channel)
        );
    }

    #[test]
    fn test_channel_next_sequence() {
        let channel = new_test_channel(1);
        assert_eq!(channel.next_sequence(), 1);
        assert_eq!(channel.next_sequence(), 2);
    }

    #[traced_test]
    #[test]
    fn test_channel_log_msg() {
        let channel = Arc::new(new_test_channel(1));
        let msg = vec![1, 2, 3];
        channel.log(&msg);
        assert!(!logs_contain(ERROR_LOGGING_MESSAGE));
    }

    #[traced_test]
    #[test]
    fn test_log_msg_success() {
        let ctx = LogContext::new();
        let recording_sink = Arc::new(RecordingSink::new());

        assert!(ctx.add_sink(recording_sink.clone()));

        let channel = new_test_channel(1);
        ctx.add_channel(channel.clone()).unwrap();
        let msg = b"test_message";

        channel.log(msg);
        assert!(!logs_contain(ERROR_LOGGING_MESSAGE));

        let recorded = recording_sink.recorded.lock();
        assert_eq!(recorded.len(), 1);
        assert_eq!(&recorded[0].channel, &channel);
        assert_eq!(recorded[0].msg, msg.to_vec());
        assert_eq!(recorded[0].metadata.sequence, 1);
        assert_eq!(
            recorded[0].metadata.log_time,
            recorded[0].metadata.publish_time
        );
        assert!(recorded[0].metadata.log_time > 1732847588055322395);
    }
}
