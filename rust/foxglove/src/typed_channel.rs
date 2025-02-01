use crate::{Channel, PartialMetadata, Schema};
use bytes::BufMut;
use schemars::{schema_for, JsonSchema};
use serde::Serialize;
use std::borrow::Cow;
use std::sync::Arc;

const STACK_BUFFER_SIZE: usize = 128 * 1024;

/// A trait representing a message that can be logged to a [`Channel`].
///
/// Implementing this trait for your type `T` enables the use of [`TypedChannel<T>`], which
/// offers a type-checked `log` method.
pub trait TypedMessage {
    type Error: std::error::Error;

    /// Returns the schema for your data.
    ///
    /// You may return `None` for rare situations where the schema is not known. Note that
    /// downstream consumers of the recording may not be able to interpret your data as a result.
    fn get_schema() -> Option<Schema>;

    /// Returns the message encoding for your data.
    ///
    /// Typically one of "protobuf" or "json".
    fn get_message_encoding() -> String;

    /// Encodes message data to the provided buffer.
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), Self::Error>;

    /// Returns an estimated encoded length for the message data.
    ///
    /// Used as a hint when allocating the buffer for [`TypedMessage::encode`].
    fn encoded_len(&self) -> Option<usize> {
        None
    }
}

impl<T: Serialize + JsonSchema> TypedMessage for T {
    type Error = serde_json::Error;

    fn get_schema() -> Option<Schema> {
        let json_schema = schema_for!(T);
        Some(Schema::new(
            std::any::type_name::<T>().to_string(),
            "jsonschema".to_string(),
            Cow::Owned(serde_json::to_vec(&json_schema).expect("Failed to serialize schema")),
        ))
    }

    fn get_message_encoding() -> String {
        "json".to_string()
    }

    fn encode(&self, buf: &mut impl BufMut) -> Result<(), Self::Error> {
        serde_json::to_writer(buf.writer(), self)
    }
}

/// A typed [`Channel`] for messages that implement [`TypedMessage`].
pub struct TypedChannel<T: TypedMessage> {
    inner: Arc<Channel>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: TypedMessage> TypedChannel<T> {
    pub(crate) fn from_channel(channel: Arc<Channel>) -> Self {
        Self {
            inner: channel,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Encodes the message and logs it on the channel.
    pub fn log(&self, msg: &T) {
        self.log_with_meta(msg, PartialMetadata::new());
    }

    /// Encodes the message and logs it on the channel with additional metadata.
    pub fn log_with_meta(&self, msg: &T, metadata: PartialMetadata) {
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

/// Registers a static [`TypedChannel`] for the provided topic and message type.
///
/// This macro is essentially just a wrapper around [`LazyLock`](std::sync::LazyLock), which
/// initializes the channel lazily upon first use. If the initialization fails (e.g., due to
/// [`FoxgloveError::DuplicateChannel`]), the program will panic.
///
/// If you don't require a static variable, you can just use
/// [`ChannelBuilder::build_typed()`](crate::ChannelBuilder::build_typed) directly.
///
/// # Example
/// ```
/// use foxglove::static_typed_channel;
/// use foxglove::schemas::{FrameTransform, SceneUpdate};
///
/// // A locally-scoped typed channel.
/// static_typed_channel!(TF, "/tf", FrameTransform);
///
/// // A pub(crate)-scoped typed channel.
/// static_typed_channel!(pub(crate) BOXES, "/boxes", SceneUpdate);
/// ```
#[macro_export]
macro_rules! static_typed_channel {
    ($vis:vis $ident: ident, $topic: literal, $ty: ty) => {
        $vis static $ident: std::sync::LazyLock<$crate::TypedChannel<$ty>> =
            std::sync::LazyLock::new(|| match $crate::ChannelBuilder::new($topic).build_typed::<$ty>() {
                Ok(channel) => channel,
                Err(e) => {
                    panic!("Failed to create channel for {}: {:?}", $topic, e);
                }
            });
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::channel_builder::ChannelBuilder;
    use crate::log_context::GlobalContextTest;
    use crate::Schema;
    use prost::bytes::BufMut;
    use serde::Serialize;
    use tracing_test::traced_test;

    #[derive(Debug, Serialize)]
    struct TestMessage {
        msg: String,
        count: u32,
    }

    impl TypedMessage for TestMessage {
        type Error = serde_json::Error;

        fn get_schema() -> Option<Schema> {
            Some(Schema::new(
                "TextMessage",
                "jsonschema",
                br#"{
                    "type": "object",
                    "properties": {
                        "msg": {"type": "string"},
                        "count": {"type": "number"},
                    },
                }"#,
            ))
        }

        fn get_message_encoding() -> String {
            "json".to_string()
        }

        fn encode(&self, buf: &mut impl BufMut) -> Result<(), Self::Error> {
            serde_json::to_writer(buf.writer(), self)
        }
    }

    #[traced_test]
    #[test]
    fn test_json_typed_channel() {
        let _cleanup = GlobalContextTest::new();
        let channel = ChannelBuilder::new("topic2")
            .build_typed::<TestMessage>()
            .expect("failed to build channel");

        let message = TestMessage {
            msg: "Hello, world!".to_string(),
            count: 42,
        };

        channel.log(&message);
        assert!(!logs_contain("error logging message"));
    }
}
