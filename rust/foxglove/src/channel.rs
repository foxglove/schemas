use foxglove_sdk_core::{FoxgloveError, LogContext, Schema};
use std::{collections::BTreeMap, sync::Arc};

pub struct Channel(Arc<foxglove_sdk_core::Channel>);

impl Channel {
    pub fn new(topic: &str, message_encoding: &str, schema: Schema) -> Result<Self, FoxgloveError> {
        Self::new_with_meta(topic, message_encoding, schema, BTreeMap::new())
    }

    pub fn new_with_meta(
        topic: &str,
        message_encoding: &str,
        schema: Schema,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, FoxgloveError> {
        let channel = foxglove_sdk_core::Channel::new(
            topic.to_string(),
            message_encoding.to_string(),
            Some(schema),
            metadata,
        );
        LogContext::global().add_channel(channel.clone())?;

        Ok(Channel(channel))
    }

    pub fn log(&self, msg: &[u8]) {
        self.0.log(msg, foxglove_sdk_core::PartialMetadata::new());
    }

    pub fn log_with_meta(&self, msg: &[u8], metadata: foxglove_sdk_core::PartialMetadata) {
        self.0.log(msg, metadata);
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.0.close();
    }
}
