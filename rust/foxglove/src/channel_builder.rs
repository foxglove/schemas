use crate::channel::ChannelId;
use crate::log_sink_set::LogSinkSet;
use crate::typed_channel::TypedChannel;
use crate::{Channel, FoxgloveError, LogContext, Schema, TypedMessage};
use std::collections::BTreeMap;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU32, AtomicU64};
use std::sync::Arc;

pub struct ChannelBuilder<'a> {
    topic: String,
    message_encoding: Option<String>,
    schema: Option<Schema>,
    metadata: BTreeMap<String, String>,
    context: Option<&'a LogContext>,
}

impl<'a> ChannelBuilder<'a> {
    pub fn new<T: Into<String>>(topic: T) -> Self {
        Self {
            topic: topic.into(),
            message_encoding: None,
            schema: None,
            metadata: BTreeMap::new(),
            context: None,
        }
    }

    pub fn schema(mut self, schema: impl Into<Option<Schema>>) -> Self {
        self.schema = schema.into();
        self
    }

    pub fn message_encoding(mut self, encoding: &str) -> Self {
        self.message_encoding = Some(encoding.to_string());
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    #[doc(hidden)]
    pub fn with_context(mut self, ctx: &'a LogContext) -> Self {
        self.context = Some(ctx);
        self
    }

    pub fn build(self) -> Result<Arc<Channel>, FoxgloveError> {
        static CHANNEL_ID: AtomicU64 = AtomicU64::new(1);
        let channel = Arc::new(Channel {
            sinks: LogSinkSet::new(),
            id: ChannelId::new(CHANNEL_ID.fetch_add(1, Relaxed)),
            message_sequence: AtomicU32::new(1),
            topic: self.topic,
            message_encoding: self
                .message_encoding
                .ok_or_else(|| FoxgloveError::Fatal("Message encoding is required".to_string()))?,
            schema: self.schema,
            metadata: self.metadata,
        });
        self.context
            .unwrap_or_else(|| LogContext::global())
            .add_channel(channel.clone())?;
        Ok(channel)
    }

    pub fn build_typed<T: TypedMessage>(mut self) -> Result<TypedChannel<T>, FoxgloveError> {
        if self.message_encoding.is_none() {
            self.message_encoding = Some(<T as TypedMessage>::get_message_encoding());
        }
        if self.schema.is_none() {
            self.schema = <T as TypedMessage>::get_schema();
        }
        let channel = self.build()?;
        Ok(TypedChannel::from_channel(channel))
    }
}
