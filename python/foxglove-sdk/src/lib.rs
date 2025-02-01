use errors::PyFoxgloveError;
use foxglove_sdk_core::log_file::FileWriter;
use foxglove_sdk_core::websocket::{self, ServerOptions};
use foxglove_sdk_core::{Channel, LogContext, LogSink, PartialMetadata, Schema};
use log::LevelFilter;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::time;

mod errors;

#[pyclass]
struct BaseChannel(Arc<Channel>);

impl Drop for BaseChannel {
    fn drop(&mut self) {
        log::info!("Channel {} dropped", self.0.id());
        // TODO we should only close the channel when the Channel itself is dropped
        self.0.close();
    }
}

#[pyclass]
struct WebSocketServer(Option<Arc<websocket::Server>>);

impl Deref for WebSocketServer {
    type Target = Arc<websocket::Server>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("WebSocket server is not running")
    }
}

impl Drop for WebSocketServer {
    fn drop(&mut self) {
        log::info!("WebSocket server dropped");
        self.stop();
    }
}

#[pymethods]
impl WebSocketServer {
    fn stop(&mut self) {
        let Some(server) = self.0.take() else {
            return;
        };
        server.stop_blocking();
        let sink: Arc<dyn LogSink> = server;
        // This is safe for now to assume that the server is attached to the default context
        LogContext::global().remove_sink(&sink);
    }
}

#[pyclass]
struct MCAPWriter(#[allow(dead_code)] Arc<FileWriter>);

impl Deref for MCAPWriter {
    type Target = Arc<FileWriter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for MCAPWriter {
    fn drop(&mut self) {
        log::info!("MCAP writer dropped");
        if let Err(err) = self.0.close() {
            log::error!("Error closing MCAP writer: {}", err);
        }
    }
}

#[pymethods]
impl BaseChannel {
    #[new]
    #[pyo3(
        signature = (topic, message_encoding, schema_name=None, schema_encoding=None, schema_data=None, metadata=None)
    )]
    fn new(
        topic: String,
        message_encoding: String,
        schema_name: Option<String>,
        schema_encoding: Option<String>,
        schema_data: Option<Vec<u8>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> PyResult<Self> {
        if (schema_data.is_some() || schema_encoding.is_some()) && schema_name.is_none() {
            return Err(PyValueError::new_err(
                "Schema name must be provided if schema data or encoding is provided.",
            ));
        }
        let channel = Channel::new(
            topic,
            message_encoding,
            schema_name.map(|name| {
                Schema::new(
                    name,
                    schema_encoding,
                    Cow::Owned(schema_data.unwrap_or_default()),
                )
            }),
            metadata.unwrap_or_default(),
        );
        LogContext::global()
            .add_channel(channel.clone())
            .map_err(PyFoxgloveError::from)?;
        Ok(BaseChannel(channel))
    }

    #[pyo3(signature = (msg, publish_time=None, log_time=None, sequence=None))]
    fn log(
        &self,
        msg: &[u8],
        publish_time: Option<u64>,
        log_time: Option<u64>,
        sequence: Option<u32>,
    ) -> PyResult<()> {
        let metadata = PartialMetadata {
            sequence,
            log_time,
            publish_time,
        };
        self.0.log(msg, metadata);
        Ok(())
    }
}

/// Open a new mcap file for recording
#[pyfunction]
#[pyo3(signature = (path))]
fn record_file(path: &str) -> PyResult<MCAPWriter> {
    let sink = FileWriter::new(Path::new(path)).map_err(PyFoxgloveError::from)?;
    LogContext::global().add_sink(sink.clone());
    Ok(MCAPWriter(sink))
}

/// Start a new Foxglove WebSocket server
#[pyfunction]
#[pyo3(signature = (name = None, host="127.0.0.1", port=0))]
fn start_server(name: Option<String>, host: &str, port: u16) -> PyResult<WebSocketServer> {
    let session_id = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Failed to create session ID; invalid system time")
        .as_millis()
        .to_string();

    let server = websocket::create_server(ServerOptions {
        session_id: Some(session_id),
        name,
        ..Default::default()
    });

    server
        .start_blocking(host, port)
        .map_err(PyFoxgloveError::from)?;

    LogContext::global().add_sink(server.clone());
    Ok(WebSocketServer(Some(server)))
}

#[pyfunction]
fn get_channel_for_topic(topic: &str) -> PyResult<Option<BaseChannel>> {
    let channel = LogContext::global().get_channel_by_topic(topic);
    Ok(channel.map(BaseChannel))
}

#[pyfunction]
fn enable_log_forwarding(level: &str) -> PyResult<()> {
    let level = match level.to_lowercase().as_str() {
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => return Err(PyErr::new::<PyValueError, _>("Invalid log level")),
    };

    log::set_max_level(level);
    Ok(())
}

#[pyfunction]
fn disable_log_forwarding() -> PyResult<()> {
    log::set_max_level(LevelFilter::Off);
    Ok(())
}

/// Our public API is in the `python` directory.
/// Rust bindings are exported as `_foxglove_py` and should not be imported directly.
#[pymodule]
fn _foxglove_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(enable_log_forwarding, m)?)?;
    m.add_function(wrap_pyfunction!(disable_log_forwarding, m)?)?;

    m.add_function(wrap_pyfunction!(record_file, m)?)?;
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    m.add_function(wrap_pyfunction!(get_channel_for_topic, m)?)?;
    m.add_class::<BaseChannel>()?;
    m.add_class::<WebSocketServer>()?;
    m.add_class::<MCAPWriter>()?;
    Ok(())
}
