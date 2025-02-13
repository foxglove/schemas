//! Service call response handling.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use bytes::Bytes;
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
    tokens: Arc<AtomicUsize>,
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
        let tokens = Arc::new(AtomicUsize::new(capacity));
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
        assert_eq!(
            self.tokens.load(Ordering::Relaxed),
            self.tx.capacity().unwrap()
        );
    }

    /// Attempts to acquire a response token.
    fn try_acquire_token(&self) -> Option<Token> {
        loop {
            let current = self.tokens.load(Ordering::Acquire);
            if current == 0 {
                return None;
            }
            if self
                .tokens
                .compare_exchange(current, current - 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return Some(Token(self.tokens.clone()));
            }
        }
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
        self.try_acquire_token().map(|token| Responder {
            service_id,
            call_id,
            encoding: encoding.to_string(),
            tx: self.tx.clone(),
            token,
        })
    }
}

/// Represents a reservation for the client's service response queue.
///
/// Increments the inner counter (a reference to [`ResponseChannel::tokens`]) when dropped.
struct Token(Arc<AtomicUsize>);

impl Drop for Token {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Release);
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
    /// Overrides the default response encoding.
    ///
    /// By default, the response encoding is the one declared in the
    /// [`ServiceSchema`][super::ServiceSchema]. If no response encoding was declared, then the
    /// encoding is presumed to be the same as the request.
    pub fn set_encoding(&mut self, encoding: impl Into<String>) {
        self.encoding = encoding.into();
    }

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
