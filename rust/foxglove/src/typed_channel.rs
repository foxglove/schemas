use crate::{schemas, Channel, FoxgloveError};
use std::collections::BTreeMap;

const STACK_BUFFER_SIZE: usize = 128 * 1024;

pub struct TypedChannel<T: schemas::TypedMessage> {
    inner: Channel,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: schemas::TypedMessage> TypedChannel<T> {
    pub fn new(topic: &str) -> Result<Self, FoxgloveError> {
        Self::new_with_meta(topic, BTreeMap::new())
    }

    pub fn new_with_meta(
        topic: &str,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, FoxgloveError> {
        Ok(TypedChannel {
            inner: Channel::new_with_meta(
                topic,
                <T as schemas::TypedMessage>::get_message_encoding(),
                <T as schemas::TypedMessage>::get_schema(),
                metadata,
            )?,
            _phantom: Default::default(),
        })
    }

    pub fn log(&self, msg: &T) {
        self.log_with_meta(msg, foxglove_sdk_core::PartialMetadata::new());
    }

    pub fn log_with_meta(&self, msg: &T, metadata: foxglove_sdk_core::PartialMetadata) {
        // Try to avoid heap allocation by using a stack buffer.
        let mut stack_buf = [0u8; STACK_BUFFER_SIZE];
        let mut cursor = &mut stack_buf[..];

        match msg.encode(&mut cursor) {
            Ok(()) => {
                // Compute the written amount of bytes
                let written = cursor.as_ptr() as usize - stack_buf.as_ptr() as usize;
                self.inner.log_with_meta(&stack_buf[..written], metadata);
            }
            Err(_) => {
                // The stack buffer was likely too small, fall back to heap allocation.
                // Unfortunately the interface of TypedMessage does not expose the size we need,
                // even though we do get that information from prost.
                // (but TypedMessage can be implemented without prost, so we keep it generic).
                let mut buf =
                    Vec::with_capacity(msg.encoded_len().unwrap_or(STACK_BUFFER_SIZE * 2));
                if let Err(err) = msg.encode(&mut buf) {
                    tracing::error!("failed to encode message: {:?}", err);
                }
                self.inner.log_with_meta(&buf, metadata);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use foxglove_sdk_core::Schema;
    use prost::bytes::BufMut;
    use serde::Serialize;
    use std::borrow::Cow;
    use tracing_test::traced_test;

    #[derive(Debug, Serialize)]
    struct TestMessage {
        msg: String,
        count: u32,
    }

    impl schemas::TypedMessage for TestMessage {
        type Error = serde_json::Error;

        fn get_schema() -> Schema {
            Schema::new(
                "TestMessage".to_string(),
                Some("json".to_string()),
                Cow::Borrowed(
                    br#"{
                "type": "object",
                "properties": {
                    "msg": {"type": "string"},
                    "count": {"type": "number"},
                },
            }"#,
                ),
            )
        }

        fn encode(&self, buf: &mut impl BufMut) -> Result<(), Self::Error> {
            serde_json::to_writer(buf.writer(), self)
        }
    }

    #[traced_test]
    #[test]
    fn test_json_typed_channel() {
        let channel = TypedChannel::<TestMessage>::new("topic").unwrap();

        let message = TestMessage {
            msg: "Hello, world!".to_string(),
            count: 42,
        };

        channel.log(&message);
        assert!(!logs_contain("error logging message"));
    }
}
