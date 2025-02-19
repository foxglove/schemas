use crate::errors::PyFoxgloveError;
use foxglove::{WebSocketServer, WebSocketServerBlockingHandle};
use pyo3::prelude::*;
use std::time;

/// Start a new Foxglove WebSocket server.
///
/// :param name: The name of the server.
/// :param host: The host to bind to.
/// :param port: The port to bind to.
/// :param capabilities: A list of capabilities to advertise to clients.
///
/// To connect to this server: open Foxglove, choose "Open a new connection", and select Foxglove
/// WebSocket. The default connection string matches the defaults used by the SDK.
#[pyfunction]
#[pyo3(signature = (name = None, host="127.0.0.1", port=8765, capabilities=None))]
pub fn start_server(
    py: Python<'_>,
    name: Option<String>,
    host: &str,
    port: u16,
    capabilities: Option<Vec<Capability>>,
) -> PyResult<PyWebSocketServer> {
    let session_id = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Failed to create session ID; invalid system time")
        .as_millis()
        .to_string();

    let mut server = WebSocketServer::new()
        .session_id(session_id)
        .bind(host, port);

    if let Some(name) = name {
        server = server.name(name);
    }

    if let Some(capabilities) = capabilities {
        server = server.capabilities(capabilities.into_iter().map(Capability::into));
    }

    let handle = py
        .allow_threads(|| server.start_blocking())
        .map_err(PyFoxgloveError::from)?;
    Ok(PyWebSocketServer(Some(handle)))
}

/// A live visualization server. Obtain an instance by calling :py:func:`start_server`.
#[pyclass(name = "WebSocketServer")]
pub struct PyWebSocketServer(pub Option<WebSocketServerBlockingHandle>);

#[pymethods]
impl PyWebSocketServer {
    pub fn stop(&mut self, py: Python<'_>) {
        if let Some(server) = self.0.take() {
            py.allow_threads(|| server.stop())
        }
    }

    /// Sets a new session ID and notifies all clients, causing them to reset their state.
    /// If no session ID is provided, generates a new one based on the current timestamp.
    #[pyo3(signature = (session_id=None))]
    pub fn clear_session(&self, session_id: Option<String>) -> PyResult<()> {
        if let Some(server) = &self.0 {
            server.clear_session(session_id);
        }
        Ok(())
    }

    pub fn broadcast_time(&self, timestamp_nanos: u64) -> PyResult<()> {
        if let Some(server) = &self.0 {
            server.broadcast_time(timestamp_nanos);
        }
        Ok(())
    }
}

/// A capability that the websocket server advertises to its clients.
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum Capability {
    /// Allow clients to advertise channels to send data messages to the server.
    // ClientPublish,
    /// Allow clients to get & set parameters.
    // Parameters,
    /// Inform clients about the latest server time.
    ///
    /// This allows accelerated, slowed, or stepped control over the progress of time. If the
    /// server publishes time data, then timestamps of published messages must originate from the
    /// same time source.
    Time,
}

impl From<Capability> for foxglove::websocket::Capability {
    fn from(value: Capability) -> Self {
        match value {
            // Capability::ClientPublish => foxglove::websocket::Capability::ClientPublish,
            // Capability::Parameters => foxglove::websocket::Capability::Parameters,
            Capability::Time => foxglove::websocket::Capability::Time,
        }
    }
}
