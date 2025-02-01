use foxglove_sdk_core::Schema;
use prost::bytes::BufMut;

pub trait TypedMessage {
    type Error: std::error::Error;
    fn get_schema() -> Schema;
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), Self::Error>;

    fn encoded_len(&self) -> Option<usize> {
        None
    }

    fn get_message_encoding<'a>() -> &'a str {
        "protobuf"
    }
}
