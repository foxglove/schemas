//! Tokio runtime.

use std::sync::LazyLock;
use std::time::Duration;

use parking_lot::Mutex;
use tokio::runtime::Handle;

struct Runtime {
    inner: Mutex<Option<tokio::runtime::Runtime>>,
    handle: Handle,
}
impl Runtime {
    fn new() -> Self {
        tracing::info!("Creating tokio runtime");
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");
        let handle = rt.handle().clone();
        Self {
            inner: Mutex::new(Some(rt)),
            handle,
        }
    }

    fn shutdown(&self) {
        let mut inner = self.inner.lock();
        if let Some(rt) = inner.take() {
            rt.shutdown_timeout(Duration::from_millis(500));
        }
    }
}

static RUNTIME: LazyLock<Runtime> = LazyLock::new(Runtime::new);

/// Returns the current runtime handle.
///
/// If there is no tokio runtime, creates a new one.
pub(crate) fn get_runtime_handle() -> Handle {
    if let Ok(handle) = Handle::try_current() {
        return handle;
    }
    RUNTIME.handle.clone()
}

/// Shuts down the tokio runtime, if we created one.
///
/// This should be called at program exit, but only if you did not create your own tokio runtime
/// (e.g. with `#[tokio::main]`).
#[doc(hidden)]
pub fn shutdown_runtime() {
    RUNTIME.shutdown()
}
