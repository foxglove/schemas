use errors::PyFoxgloveError;
use foxglove::{
    Channel, ChannelBuilder, LogContext, McapWriter, McapWriterHandle, PartialMetadata, Schema,
    WebSocketServer, WebSocketServerBlockingHandle,
};
use log::LevelFilter;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use std::time;

mod errors;

#[pyclass]
struct BaseChannel(Arc<Channel>);

#[pyclass]
struct PyWebSocketServer(Option<WebSocketServerBlockingHandle>);

#[pymethods]
impl PyWebSocketServer {
    fn stop(&mut self, py: Python<'_>) {
        if let Some(server) = self.0.take() {
            py.allow_threads(|| server.stop())
        }
    }
}

#[pyclass]
struct PyMcapWriter(Option<McapWriterHandle<BufWriter<File>>>);

impl Drop for PyMcapWriter {
    fn drop(&mut self) {
        log::info!("MCAP writer dropped");
        if let Err(e) = self.close() {
            log::error!("Failed to close MCAP writer: {e}");
        }
    }
}

#[pymethods]
impl PyMcapWriter {
    fn close(&mut self) -> PyResult<()> {
        if let Some(writer) = self.0.take() {
            writer.close().map_err(PyFoxgloveError::from)?;
        }
        Ok(())
    }
}

#[pymethods]
impl BaseChannel {
    #[new]
    #[pyo3(
        signature = (topic, message_encoding, schema_name, schema_encoding=None, schema_data=None, metadata=None)
    )]
    fn new(
        topic: &str,
        message_encoding: &str,
        schema_name: Option<String>,
        schema_encoding: Option<String>,
        schema_data: Option<Vec<u8>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> PyResult<Self> {
        let schema = match (
            schema_name,
            schema_encoding.filter(|s| !s.is_empty()),
            schema_data.filter(|s| !s.is_empty()),
        ) {
            (Some(name), Some(encoding), Some(data)) => Some(Schema::new(name, encoding, data)),
            (_, None, None) => None,
            (_, None, Some(_)) => {
                return Err(PyValueError::new_err(
                    "Schema encoding must be provided if schema data is provided.",
                ));
            }
            (_, Some(_), None) => {
                return Err(PyValueError::new_err(
                    "Schema data must be provided if schema encoding is provided.",
                ));
            }
            _ => {
                return Err(PyValueError::new_err(
                    "Schema name must be provided if schema data or encoding is provided.",
                ));
            }
        };

        let channel = ChannelBuilder::new(topic)
            .message_encoding(message_encoding)
            .schema(schema)
            .metadata(metadata.unwrap_or_default())
            .build()
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
        self.0.log_with_meta(msg, metadata);
        Ok(())
    }
}

/// Open a new mcap file for recording
#[pyfunction]
#[pyo3(signature = (path))]
fn record_file(path: &str) -> PyResult<PyMcapWriter> {
    let handle = McapWriter::new()
        .create_new_buffered_file(path)
        .map_err(PyFoxgloveError::from)?;
    Ok(PyMcapWriter(Some(handle)))
}

/// Start a new Foxglove WebSocket server
#[pyfunction]
#[pyo3(signature = (name = None, host="127.0.0.1", port=0))]
fn start_server(
    py: Python<'_>,
    name: Option<String>,
    host: &str,
    port: u16,
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

    let handle = py
        .allow_threads(|| server.start_blocking())
        .map_err(PyFoxgloveError::from)?;
    Ok(PyWebSocketServer(Some(handle)))
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

#[pyfunction]
fn shutdown(py: Python<'_>) {
    py.allow_threads(foxglove::shutdown_runtime);
}

/// Our public API is in the `python` directory.
/// Rust bindings are exported as `_foxglove_py` and should not be imported directly.
#[pymodule]
fn _foxglove_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(enable_log_forwarding, m)?)?;
    m.add_function(wrap_pyfunction!(disable_log_forwarding, m)?)?;
    m.add_function(wrap_pyfunction!(shutdown, m)?)?;
    m.add_function(wrap_pyfunction!(record_file, m)?)?;
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    m.add_function(wrap_pyfunction!(get_channel_for_topic, m)?)?;
    m.add_class::<BaseChannel>()?;
    m.add_class::<PyWebSocketServer>()?;
    m.add_class::<PyMcapWriter>()?;
    Ok(())
}
