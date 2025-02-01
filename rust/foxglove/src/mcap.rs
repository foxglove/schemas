//! MCAP writer

use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

use foxglove_sdk_core::{FileWriter, FoxgloveError, LogContext, LogSink};

/// An MCAP file for logging events to disk.
#[must_use]
#[derive(Debug)]
pub struct McapWriter {
    path: PathBuf,
}

impl McapWriter {
    /// Instantiate a new MCAP file at the specified path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Initializes the MCAP file and registers it to receive events.
    ///
    /// Returns a handle. When the handle is dropped, the file will be flushed and closed. The
    /// caller can also use the handle to explicitly close the file in order to check for errors.
    pub fn create(self) -> Result<McapWriterHandle, FoxgloveError> {
        let writer = FileWriter::new(&self.path)?;
        LogContext::global().add_sink(writer.clone());
        Ok(McapWriterHandle(writer))
    }
}

/// A handle to an MCAP file writer.
///
/// When this handle is dropped, the file will be flushed and closed.
#[must_use]
pub struct McapWriterHandle(Arc<FileWriter>);

impl Debug for McapWriterHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("McapWriterHandle").finish_non_exhaustive()
    }
}

impl McapWriterHandle {
    /// Flushes data to the file and closes it.
    pub fn close(self) -> Result<(), FoxgloveError> {
        self.do_close()
    }

    fn do_close(&self) -> Result<(), FoxgloveError> {
        let sink = self.0.clone() as Arc<dyn LogSink>;
        LogContext::global().remove_sink(&sink);
        self.0.close()
    }
}

impl Drop for McapWriterHandle {
    fn drop(&mut self) {
        if let Err(e) = self.do_close() {
            tracing::warn!("{e}");
        }
    }
}
