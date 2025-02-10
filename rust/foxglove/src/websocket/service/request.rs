//! Websocket service request.

use bytes::Bytes;

use super::{CallId, ServiceId};

/// A service call request.
#[derive(Debug, Clone)]
pub struct Request {
    /// Service ID.
    pub service_id: ServiceId,
    /// Service name.
    pub service_name: String,
    /// Call ID.
    pub call_id: CallId,
    /// Encoding for the request data.
    pub encoding: String,
    /// Request data.
    pub payload: Bytes,
}
