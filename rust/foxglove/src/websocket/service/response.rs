//! Service call response handling.

use std::sync::Arc;

use bytes::Bytes;
use parking_lot::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tracing::error;

use crate::websocket::protocol;
use crate::websocket::service::{CallId, ServiceId};

#[cfg(test)]
mod tests;

/// The default maximum number of concurrent requests from a particular client.
const DEFAULT_RESPONSE_CHANNEL_CAPACITY: usize = 32;

/// A bounded response channel.
pub(crate) struct ResponseChannel {
    tokens: Arc<Mutex<usize>>,
    tx: flume::Sender<Response>,
    rx: flume::Receiver<Response>,
}
impl Default for ResponseChannel {
    fn default() -> Self {
        Self::new(DEFAULT_RESPONSE_CHANNEL_CAPACITY)
    }
}
impl ResponseChannel {
    /// Constructs a new service response channel.
    ///
    /// The service response channel is configured with a capacity, which represents the maximum
    /// number of concurrent service calls from a particular client.
    pub fn new(capacity: usize) -> Self {
        let tokens = Arc::new(Mutex::new(capacity));
        let (tx, rx) = flume::bounded(capacity);
        Self { tokens, tx, rx }
    }

    /// Removes a response message from the queue.
    pub async fn next_message(&self) -> Message {
        // By converting the `Response` to a `Message`, we're also dropping the `Token`.
        self.rx
            .recv_async()
            .await
            .map(Message::from)
            .expect("We're holding the sender")
    }

    /// Removes all responses from the channel.
    pub fn drain(&self) {
        self.rx.drain();
        #[cfg(debug_assertions)]
        assert_eq!(*self.tokens.lock(), self.tx.capacity().unwrap());
    }

    /// Allocates a response token and constructs a [`Responder`].
    ///
    /// Returns `None` if there are too many concurrent requests.
    ///
    /// The responder holds ownership of a [`Token`], which represents a reservation in the
    /// bounded response channel. This token rides along into the [`Response`], and is finally
    /// dropped when it is removed from the queue.
    pub fn prepare_response(
        &self,
        service_id: ServiceId,
        call_id: CallId,
        encoding: &str,
    ) -> Option<Responder> {
        let mut tokens = self.tokens.lock();
        if *tokens > 0 {
            *tokens -= 1;
            Some(Responder {
                service_id,
                call_id,
                encoding: encoding.to_string(),
                tx: self.tx.clone(),
                token: Token(self.tokens.clone()),
            })
        } else {
            None
        }
    }
}

/// Represents a reservation for the client's service response queue.
///
/// Increments the inner counter (a reference to [`ResponseChannel::tokens`]) when dropped.
struct Token(Arc<Mutex<usize>>);

impl Drop for Token {
    fn drop(&mut self) {
        let mut tokens = self.0.lock();
        #[cfg(debug_assertions)]
        assert_ne!(*tokens, usize::MAX);
        *tokens += 1;
    }
}

/// A handle for completing a service call.
///
/// If you're holding one of these, you're responsible for eventually calling
/// [`Responder::respond`]. If you drop the responder without responding, the client will never
/// receive a response for its request.
#[must_use]
pub struct Responder {
    service_id: ServiceId,
    call_id: CallId,
    encoding: String,
    tx: flume::Sender<Response>,
    token: Token,
}
impl Responder {
    /// Completes the request by sending a response to the client.
    pub fn respond(self, result: Result<Bytes, String>) {
        let response = Response {
            service_id: self.service_id,
            call_id: self.call_id,
            encoding: self.encoding,
            result,
            _token: self.token,
        };
        if let Err(e) = self.tx.try_send(response) {
            match e {
                flume::TrySendError::Full(_) => {
                    error!("Service response channel full");
                    #[cfg(debug_assertions)]
                    unreachable!("We're holding a response token");
                }
                flume::TrySendError::Disconnected(_) => (),
            }
        }
    }
}

/// A service call response.
struct Response {
    service_id: ServiceId,
    call_id: CallId,
    encoding: String,
    result: Result<Bytes, String>,
    _token: Token,
}
impl From<Response> for Message {
    fn from(r: Response) -> Self {
        match r.result {
            Ok(payload) => Message::binary(
                protocol::server::ServiceCallResponse::new(
                    r.service_id,
                    r.call_id,
                    r.encoding,
                    payload,
                )
                .encode(),
            ),
            Err(message) => Message::text(protocol::server::service_call_failure(
                r.service_id,
                r.call_id,
                &message,
            )),
        }
    }
}
