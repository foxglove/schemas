//! Websocket services.

use std::fmt::Display;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::websocket::Client;

mod handler;
mod request;
mod response;
mod schema;
use bytes::Bytes;
pub use handler::{Handler, SyncHandler};
use handler::{HandlerFn, SyncHandlerFn};
pub use request::Request;
pub use response::Responder;
pub(crate) use response::ResponseChannel;
pub(crate) use schema::MessageSchema;
pub use schema::ServiceSchema;

/// A service ID, which uniquely identifies a service hosted by the server.
pub type ServiceId = u32;

/// A service call ID, which uniquely identifies an outstanding call for a particular client.
pub type CallId = u32;

/// A builder for a websocket service.
#[must_use]
#[derive(Debug)]
pub struct ServiceBuilder {
    id: ServiceId,
    name: String,
    schema: ServiceSchema,
}
impl ServiceBuilder {
    /// Creates a new builder for a websocket service.
    fn new(name: impl Into<String>, schema: ServiceSchema) -> Self {
        static ID: AtomicU32 = AtomicU32::new(1);
        let id = ID.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            name: name.into(),
            schema,
        }
    }

    /// Allow overriding the ID for deterministic tests.
    #[cfg(test)]
    pub(crate) fn with_id(mut self, id: ServiceId) -> Self {
        self.id = id;
        self
    }

    /// Configures a handler and returns the constructed [`Service`].
    pub fn handler<H: Handler + 'static>(self, handler: H) -> Service {
        Service {
            id: self.id,
            name: self.name,
            schema: self.schema,
            handler: Arc::new(handler),
        }
    }

    /// Configures a handler function and returns the constructed [`Service`].
    ///
    /// Refer to [`Handler::call`] for a description of the `call` function.
    pub fn handler_fn<F>(self, call: F) -> Service
    where
        F: Fn(Client, Request, Responder) + Send + Sync + 'static,
    {
        self.handler(HandlerFn(call))
    }

    /// Configures a synchronous handler function and returns the constructed [`Service`].
    ///
    /// Refer to [`SyncHandler::call`] for a description of the `call` function.
    pub fn sync_handler_fn<F, E>(self, call: F) -> Service
    where
        F: Fn(Client, Request) -> Result<Bytes, E> + Send + Sync + 'static,
        E: Display + 'static,
    {
        self.handler(SyncHandlerFn(call))
    }
}

/// A websocket service.
#[must_use]
pub struct Service {
    id: ServiceId,
    name: String,
    schema: ServiceSchema,
    handler: Arc<dyn Handler>,
}

impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Service")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("schema", &self.schema)
            .finish_non_exhaustive()
    }
}

impl Service {
    /// Creates a new builder for a websocket service.
    pub fn builder(name: impl Into<String>, schema: ServiceSchema) -> ServiceBuilder {
        ServiceBuilder::new(name, schema)
    }

    /// Returns the service's ID.
    pub fn id(&self) -> ServiceId {
        self.id
    }

    /// Returns the service's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the service schema.
    pub fn schema(&self) -> &ServiceSchema {
        &self.schema
    }

    pub(crate) fn request_encoding(&self) -> Option<&str> {
        self.schema().request().map(|rs| rs.encoding.as_str())
    }

    pub(crate) fn response_encoding(&self) -> Option<&str> {
        self.schema().response().map(|rs| rs.encoding.as_str())
    }

    /// Invokes the service call implementation.
    pub(crate) fn call(
        &self,
        client: Client<'_>,
        call_id: CallId,
        encoding: String,
        payload: Bytes,
        responder: Responder,
    ) {
        let request = Request {
            service_id: self.id,
            service_name: self.name.clone(),
            call_id,
            encoding: encoding.clone(),
            payload,
        };
        self.handler.call(client, request, responder);
    }
}
