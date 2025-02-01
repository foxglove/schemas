pub use foxglove_sdk_core::{FoxgloveError, Schema};

mod channel;
pub use channel::Channel;
mod mcap;
pub use mcap::{McapWriter, McapWriterHandle};
pub mod schemas;
mod typed_channel;

mod websocket;
pub use websocket::{WebSocketServer, WebSocketServerHandle};
