//! Tokio runtime.

use std::sync::LazyLock;

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
        self.inner.lock().take();
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

/// Shuts down the tokio runtime, ensuring that there are no remaining async tasks.
///
/// This function is a no-op if we didn't create our own internal tokio runtime.
///
/// This function should only be used as part of a graceful program shutdown.
///
/// Typically it isn't necessary to shutdown the runtime explicitly, but under some circumstances
/// it can be useful to ensure that there are no more async tasks running.
///
/// This function will block forever waiting for async tasks to yield. Tasks are not guaranteed to
/// run until completion, but might do so if they do not yield until completion.
///
/// Once the runtime is shut down, it will not be restarted or replaced.
#[doc(hidden)]
pub fn shutdown_runtime() {
    RUNTIME.shutdown()
}
