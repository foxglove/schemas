//! Websocket server

use std::fmt::Debug;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::websocket::{create_server, Server, ServerOptions};
use crate::{FoxgloveError, LogContext, LogSink};

/// A websocket server for live visualization.
#[must_use]
#[derive(Debug)]
pub struct WebSocketServer {
    host: String,
    port: u16,
    options: ServerOptions,
}

impl Default for WebSocketServer {
    fn default() -> Self {
        let session_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|d| d.as_millis().to_string());
        let options = ServerOptions {
            session_id,
            ..ServerOptions::default()
        };
        Self {
            host: "127.0.0.1".into(),
            port: 8765,
            options,
        }
    }
}

impl WebSocketServer {
    /// Creates a new websocket server with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the websocket server name to advertise to clients.
    ///
    /// By default, the server is not given a name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.options.name = Some(name.into());
        self
    }

    /// Bind a TCP port.
    ///
    /// By default, the server will bind to `127.0.0.1:8765`.
    pub fn bind(mut self, host: impl Into<String>, port: u16) -> Self {
        self.host = host.into();
        self.port = port;
        self
    }

    /// Set the message backlog size.
    ///
    /// The server buffers outgoing log entries into a queue. If the backlog size is exceeded, the
    /// oldest entries will be dropped.
    ///
    /// By default, the server will buffer 1024 messages.
    pub fn message_backlog_size(mut self, size: usize) -> Self {
        self.options.message_backlog_size = Some(size);
        self
    }

    /// Set a session ID.
    ///
    /// This allows the client to understand if the connection is a re-connection or if it is
    /// connecting to a new server instance. This can for example be a timestamp or a UUID.
    ///
    /// By default, this is set to the number of milliseconds since the unix epoch.
    pub fn session_id(mut self, id: impl Into<String>) -> Self {
        self.options.session_id = Some(id.into());
        self
    }

    /// Starts the websocket server.
    ///
    /// Returns a handle that can optionally be used to gracefully shutdown the server. The caller
    /// can safely drop the handle, and the server will run forever.
    pub async fn start(self) -> Result<WebSocketServerHandle, FoxgloveError> {
        let server = create_server(self.options);
        server.start(&self.host, self.port).await?;
        LogContext::global().add_sink(server.clone());
        Ok(WebSocketServerHandle(server))
    }

    #[doc(hidden)]
    pub fn start_blocking(self) -> Result<WebSocketServerHandle, FoxgloveError> {
        let handle = tokio::runtime::Handle::current();
        handle.block_on(self.start())
    }
}

/// A handle to the websocket server.
///
/// This handle can safely be dropped and the server will run forever.
pub struct WebSocketServerHandle(Arc<Server>);

impl Debug for WebSocketServerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WebSocketServerHandle").finish()
    }
}

impl WebSocketServerHandle {
    /// Gracefully shutdown the websocket server.
    pub async fn stop(self) {
        let sink = self.0.clone() as Arc<dyn LogSink>;
        LogContext::global().remove_sink(&sink);
        self.0.stop().await;
    }

    #[doc(hidden)]
    pub fn stop_blocking(self) {
        let handle = self.0.runtime_handle.clone();
        handle.block_on(self.stop());
    }
}
