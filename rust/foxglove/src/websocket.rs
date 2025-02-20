//! Websocket functionality

use crate::channel::ChannelId;
use crate::cow_vec::CowVec;
pub(crate) use crate::websocket::protocol::client::{
    ClientChannel, ClientChannelId, ClientMessage, Subscription, SubscriptionId,
};
pub use crate::websocket::protocol::server::{
    Capability, Parameter, ParameterType, ParameterValue, Status, StatusLevel,
};
use crate::{get_runtime_handle, Channel, FoxgloveError, LogSink, Metadata};
use bimap::BiHashMap;
use bytes::{BufMut, BytesMut};
use flume::TrySendError;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use std::collections::hash_map::Entry;
use std::collections::HashSet;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed};
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Weak;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::runtime::Handle;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_tungstenite::{
    tungstenite::{self, handshake::server, http::HeaderValue, Message},
    WebSocketStream,
};
use tokio_util::sync::CancellationToken;

mod protocol;
pub mod service;
#[cfg(test)]
mod tests;
#[cfg(all(test, feature = "unstable"))]
mod unstable_tests;

use service::{CallId, Service, ServiceId};

/// Identifies a client connection. Unique for the duration of the server's lifetime.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ClientId(u32);

impl From<ClientId> for u32 {
    fn from(client: ClientId) -> Self {
        client.0
    }
}

/// A connected client session with the websocket server.
#[derive(Debug)]
pub struct Client<'a>(&'a ConnectedClient);

impl Client<'_> {
    /// Returns the client ID.
    pub fn id(&self) -> ClientId {
        self.0.id
    }
}

/// Information about a client channel.
#[derive(Debug)]
pub struct ClientChannelView<'a> {
    id: ClientChannelId,
    topic: &'a str,
}

impl ClientChannelView<'_> {
    /// Returns the client channel ID.
    pub fn id(&self) -> ClientChannelId {
        self.id
    }

    /// Returns the topic of the client channel.
    pub fn topic(&self) -> &str {
        self.topic
    }
}

/// Information about a channel.
#[derive(Debug)]
pub struct ChannelView<'a> {
    id: ChannelId,
    topic: &'a str,
}

impl ChannelView<'_> {
    /// Returns the channel ID.
    pub fn id(&self) -> ChannelId {
        self.id
    }

    /// Returns the topic of the channel.
    pub fn topic(&self) -> &str {
        self.topic
    }
}

pub(crate) const SUBPROTOCOL: &str = "foxglove.sdk.v1";
const MAX_SEND_RETRIES: usize = 10;

type WebsocketSender = SplitSink<WebSocketStream<TcpStream>, Message>;

// Queue up to 1024 messages per connected client before dropping messages
const DEFAULT_MESSAGE_BACKLOG_SIZE: usize = 1024;
const DEFAULT_CONTROL_PLANE_BACKLOG_SIZE: usize = 64;
const DEFAULT_SERVICE_CALLS_PER_CLIENT: usize = 32;

#[derive(Error, Debug)]
enum WSError {
    #[error("client handshake failed")]
    HandshakeError,
}

#[derive(Default)]
pub(crate) struct ServerOptions {
    pub session_id: Option<String>,
    pub name: Option<String>,
    pub message_backlog_size: Option<usize>,
    pub listener: Option<Arc<dyn ServerListener>>,
    pub capabilities: Option<HashSet<Capability>>,
    pub services: HashMap<String, Service>,
    pub supported_encodings: Option<HashSet<String>>,
    pub runtime: Option<Handle>,
}

impl std::fmt::Debug for ServerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerOptions")
            .field("session_id", &self.session_id)
            .field("name", &self.name)
            .field("message_backlog_size", &self.message_backlog_size)
            .field("services", &self.services)
            .finish()
    }
}

/// A websocket server that implements the Foxglove WebSocket Protocol
pub(crate) struct Server {
    /// A weak reference to the Arc holding the server.
    /// This is used to get a reference to the outer `Arc<Server>` from Server methods.
    /// See the arc() method and its callers. We need the Arc so we can use it in async futures
    /// which need to prove to the compiler that the server will outlive the future.
    /// It's analogous to the mixin shared_from_this in C++.
    weak_self: Weak<Self>,
    started: AtomicBool,
    message_backlog_size: u32,
    runtime: Handle,
    /// May be provided by the caller
    session_id: parking_lot::RwLock<String>,
    name: String,
    clients: CowVec<Arc<ConnectedClient>>,
    channels: parking_lot::RwLock<HashMap<ChannelId, Arc<Channel>>>,
    /// Callbacks for handling client messages, etc.
    listener: Option<Arc<dyn ServerListener>>,
    /// Capabilities advertised to clients
    capabilities: HashSet<Capability>,
    /// Parameters subscribed to by clients
    subscribed_parameters: parking_lot::Mutex<HashSet<String>>,
    /// Encodings server can accept from clients. Ignored unless the "clientPublish" capability is set.
    supported_encodings: HashSet<String>,
    /// Token for cancelling all tasks
    cancellation_token: CancellationToken,
    /// Registered services.
    services: parking_lot::RwLock<HashMap<ServiceId, Arc<Service>>>,
}

/// Provides a mechanism for registering callbacks for handling client message events.
///
/// These methods are invoked from the client's main poll loop and must not block. If blocking or
/// long-running behavior is required, the implementation should use [`tokio::task::spawn`] (or
/// [`tokio::task::spawn_blocking`]).
pub trait ServerListener: Send + Sync {
    /// Callback invoked when a client message is received.
    fn on_message_data(
        &self,
        _client: Client,
        _client_channel: ClientChannelView,
        _payload: &[u8],
    ) {
    }
    /// Callback invoked when a client subscribes to a channel.
    /// Only invoked if the channel is associated with the server and isn't already subscribed to by the client.
    fn on_subscribe(&self, _client: Client, _channel: ChannelView) {}
    /// Callback invoked when a client unsubscribes from a channel.
    /// Only invoked for channels that had an active subscription from the client.
    fn on_unsubscribe(&self, _client: Client, _channel: ChannelView) {}
    /// Callback invoked when a client advertises a client channel. Requires [`Capability::ClientPublish`].
    fn on_client_advertise(&self, _client: Client, _channel: ClientChannelView) {}
    /// Callback invoked when a client unadvertises a client channel. Requires [`Capability::ClientPublish`].
    fn on_client_unadvertise(&self, _client: Client, _channel: ClientChannelView) {}
    /// Callback invoked when a client requests parameters. Requires [`Capability::Parameters`].
    /// Should return the named paramters, or all paramters if param_names is empty.
    fn on_get_parameters(
        &self,
        _client: Client,
        _param_names: Vec<String>,
        _request_id: Option<&str>,
    ) -> Vec<Parameter> {
        Vec::new()
    }
    /// Callback invoked when a client sets parameters. Requires [`Capability::Parameters`].
    /// Should return the updated parameters for the passed parameters.
    /// The implementation could return the modified parameters.
    /// All clients subscribed to updates for the _returned_ parameters will be notified.
    fn on_set_parameters(
        &self,
        _client: Client,
        parameters: Vec<Parameter>,
        _request_id: Option<&str>,
    ) -> Vec<Parameter> {
        parameters
    }
    /// Callback invoked when a client subscribes to parameters. Requires [`Capability::ParametersSubscribe`].
    fn on_parameters_subscribe(&self, _param_names: Vec<String>) {}
    /// Callback invoked when a client unsubscribes from parameters. Requires [`Capability::ParametersSubscribe`].
    fn on_parameters_unsubscribe(&self, _param_names: Vec<String>) {}
}

/// A connected client session with the websocket server.
pub(crate) struct ConnectedClient {
    id: ClientId,
    addr: SocketAddr,
    weak_self: Weak<Self>,
    /// Write side of a WS stream
    sender: Mutex<WebsocketSender>,
    data_plane_tx: flume::Sender<Message>,
    data_plane_rx: flume::Receiver<Message>,
    control_plane_tx: flume::Sender<Message>,
    control_plane_rx: flume::Receiver<Message>,
    service_call_sem: service::Semaphore,
    /// Subscriptions from this client
    subscriptions: parking_lot::Mutex<BiHashMap<ChannelId, SubscriptionId>>,
    /// Channels advertised by this client
    advertised_channels: parking_lot::Mutex<HashMap<ClientChannelId, Arc<ClientChannel>>>,
    /// Parameters subscribed to by this client
    parameter_subscriptions: parking_lot::Mutex<HashSet<String>>,
    /// Optional callback handler for a server implementation
    server_listener: Option<Arc<dyn ServerListener>>,
    server: Weak<Server>,
}

impl ConnectedClient {
    fn arc(&self) -> Arc<Self> {
        self.weak_self
            .upgrade()
            .expect("client cannot be dropped while in use")
    }

    /// Handle a text or binary message sent from the client.
    ///
    /// Standard protocol messages (such as Close) should be handled upstream.
    fn handle_message(&self, message: Message) {
        let parse_result = match message {
            Message::Text(bytes) => ClientMessage::parse_json(bytes.as_str()),
            Message::Binary(bytes) => match ClientMessage::parse_binary(bytes) {
                Err(e) => Err(e),
                Ok(Some(msg)) => Ok(msg),
                Ok(None) => {
                    tracing::debug!("Received empty binary message from {}", self.addr);
                    return;
                }
            },
            _ => {
                tracing::debug!("Unhandled websocket message: {message:?}");
                return;
            }
        };
        let msg = match parse_result {
            Ok(msg) => msg,
            Err(err) => {
                tracing::error!("Invalid message from {}: {err}", self.addr);
                self.send_error(format!("Invalid message: {err}"));
                return;
            }
        };
        let Some(server) = self.server.upgrade() else {
            return;
        };

        match msg {
            ClientMessage::Subscribe(msg) => self.on_subscribe(server, msg.subscriptions),
            ClientMessage::Unsubscribe(msg) => self.on_unsubscribe(server, msg.subscription_ids),
            ClientMessage::Advertise(msg) => self.on_advertise(server, msg.channels),
            ClientMessage::Unadvertise(msg) => self.on_unadvertise(msg.channel_ids),
            ClientMessage::MessageData(msg) => self.on_message_data(msg),
            ClientMessage::GetParameters(msg) => {
                self.on_get_parameters(server, msg.parameter_names, msg.id)
            }
            ClientMessage::SetParameters(msg) => {
                self.on_set_parameters(server, msg.parameters, msg.id)
            }
            ClientMessage::SubscribeParameterUpdates(msg) => {
                self.on_parameters_subscribe(server, msg.parameter_names)
            }
            ClientMessage::UnsubscribeParameterUpdates(msg) => {
                self.on_parameters_unsubscribe(server, msg.parameter_names)
            }
            ClientMessage::ServiceCallRequest(msg) => self.on_service_call(msg),
            _ => {
                tracing::error!("Unsupported message from {}: {}", self.addr, msg.op());
                self.send_error(format!("Unsupported message: {}", msg.op()));
            }
        }
    }

    /// Send the message on the data plane, dropping up to retries older messages to make room, if necessary.
    fn send_data_lossy(&self, message: Message, retries: usize) -> SendLossyResult {
        send_lossy(
            &self.addr,
            &self.data_plane_tx,
            &self.data_plane_rx,
            message,
            retries,
        )
    }

    /// Send the message on the control plane, disconnecting the client if the channel is full.
    fn send_control_msg(&self, message: Message) -> bool {
        if let Err(TrySendError::Full(_)) = self.control_plane_tx.try_send(message) {
            // TODO disconnect the slow client FG-10441
            tracing::error!(
                "Client control plane is full for {}, dropping message",
                self.addr
            );
            return false;
        }
        true
    }

    fn on_disconnect(&self, server: &Arc<Server>) {
        // If we track paramter subscriptions, unsubscribe this clients subscriptions
        // and notify the handler, if necessary
        if !server
            .capabilities
            .contains(&Capability::ParametersSubscribe)
            || self.server_listener.is_none()
        {
            return;
        }

        // Remove all subscriptions from the server subscriptions.
        // First take the server-wide lock.
        let mut all_subscriptions = server.subscribed_parameters.lock();

        // Remove the parameter subscriptions for this client,
        // and filter out any we weren't subscribed to.
        let mut client_subscriptions = self.parameter_subscriptions.lock();
        let client_subscriptions = std::mem::take(&mut *client_subscriptions);
        let mut unsubscribed_parameters =
            server.parameters_without_subscription(client_subscriptions.into_iter().collect());
        if unsubscribed_parameters.is_empty() {
            return;
        }

        unsubscribed_parameters.retain(|name| all_subscriptions.remove(name));
        if let Some(handler) = self.server_listener.as_ref() {
            handler.on_parameters_unsubscribe(unsubscribed_parameters);
        }
    }

    fn on_message_data(&self, message: protocol::client::ClientMessageData) {
        let channel_id = message.channel_id;
        let payload = message.payload;
        let client_channel = {
            let advertised_channels = self.advertised_channels.lock();
            let Some(channel) = advertised_channels.get(&channel_id) else {
                tracing::error!("Received message for unknown channel: {}", channel_id);
                self.send_error(format!("Unknown channel ID: {}", channel_id));
                // Do not forward to server listener
                return;
            };
            channel.clone()
        };
        // Call the handler after releasing the advertised_channels lock
        if let Some(handler) = self.server_listener.as_ref() {
            handler.on_message_data(
                Client(self),
                ClientChannelView {
                    id: client_channel.id,
                    topic: &client_channel.topic,
                },
                &payload,
            );
        }
    }

    fn on_unadvertise(&self, mut channel_ids: Vec<ClientChannelId>) {
        let mut client_channels = Vec::with_capacity(channel_ids.len());
        // Using a limited scope and iterating twice to avoid holding the lock on advertised_channels while calling on_client_unadvertise
        {
            let mut advertised_channels = self.advertised_channels.lock();
            let mut i = 0;
            while i < channel_ids.len() {
                let id = channel_ids[i];
                let Some(channel) = advertised_channels.remove(&id) else {
                    // Remove the channel ID from the list so we don't invoke the on_client_unadvertise callback
                    channel_ids.swap_remove(i);
                    self.send_warning(format!(
                        "Client is not advertising channel: {}; ignoring unadvertisement",
                        id
                    ));
                    continue;
                };
                client_channels.push(channel.clone());
                i += 1;
            }
        }
        // Call the handler after releasing the advertised_channels lock
        if let Some(handler) = self.server_listener.as_ref() {
            for (id, client_channel) in channel_ids.iter().cloned().zip(client_channels) {
                handler.on_client_unadvertise(
                    Client(self),
                    ClientChannelView {
                        id,
                        topic: &client_channel.topic,
                    },
                );
            }
        }
    }

    fn on_advertise(&self, server: Arc<Server>, channels: Vec<ClientChannel>) {
        if !server.capabilities.contains(&Capability::ClientPublish) {
            self.send_error("Server does not support clientPublish capability".to_string());
            return;
        }

        for channel in channels {
            // Using a limited scope here to avoid holding the lock on advertised_channels while calling on_client_advertise
            let client_channel = {
                match self.advertised_channels.lock().entry(channel.id) {
                    Entry::Occupied(_) => {
                        self.send_warning(format!(
                            "Client is already advertising channel: {}; ignoring advertisement",
                            channel.id
                        ));
                        continue;
                    }
                    Entry::Vacant(entry) => {
                        let client_channel = Arc::new(channel);
                        entry.insert(client_channel.clone());
                        client_channel
                    }
                }
            };

            // Call the handler after releasing the advertised_channels lock
            if let Some(handler) = self.server_listener.as_ref() {
                handler.on_client_advertise(
                    Client(self),
                    ClientChannelView {
                        id: client_channel.id,
                        topic: &client_channel.topic,
                    },
                );
            }
        }
    }

    fn on_unsubscribe(&self, server: Arc<Server>, subscription_ids: Vec<SubscriptionId>) {
        let mut unsubscribed_channel_ids = Vec::with_capacity(subscription_ids.len());
        // First gather the unsubscribed channel ids while holding the subscriptions lock
        {
            let mut subscriptions = self.subscriptions.lock();
            for subscription_id in subscription_ids {
                if let Some((channel_id, _)) = subscriptions.remove_by_right(&subscription_id) {
                    unsubscribed_channel_ids.push(channel_id);
                }
            }
        }

        // If we don't have a ServerListener, we're done.
        let Some(handler) = self.server_listener.as_ref() else {
            return;
        };

        // Then gather the actual channel references while holding the channels lock
        let mut unsubscribed_channels = Vec::with_capacity(unsubscribed_channel_ids.len());
        {
            let channels = server.channels.read();
            for channel_id in unsubscribed_channel_ids {
                if let Some(channel) = channels.get(&channel_id) {
                    unsubscribed_channels.push(channel.clone());
                }
            }
        }

        // Finally call the handler for each channel
        for channel in unsubscribed_channels {
            handler.on_unsubscribe(
                Client(self),
                ChannelView {
                    id: channel.id,
                    topic: &channel.topic,
                },
            );
        }
    }

    fn on_subscribe(&self, server: Arc<Server>, mut subscriptions: Vec<Subscription>) {
        // First prune out any subscriptions for channels not in the channel map,
        // limiting how long we need to hold the lock.
        let mut subscribed_channels = Vec::with_capacity(subscriptions.len());
        {
            let channels = server.channels.read();
            let mut i = 0;
            while i < subscriptions.len() {
                let subscription = &subscriptions[i];
                let Some(channel) = channels.get(&subscription.channel_id) else {
                    tracing::error!(
                        "Client {} attempted to subscribe to unknown channel: {}",
                        self.addr,
                        subscription.channel_id
                    );
                    self.send_error(format!("Unknown channel ID: {}", subscription.channel_id));
                    // Remove the subscription from the list so we don't invoke the on_subscribe callback for it
                    subscriptions.swap_remove(i);
                    continue;
                };
                subscribed_channels.push(channel.clone());
                i += 1
            }
        }

        for (subscription, channel) in subscriptions.into_iter().zip(subscribed_channels) {
            // Using a limited scope here to avoid holding the lock on subscriptions while calling on_subscribe
            {
                let mut subscriptions = self.subscriptions.lock();
                if subscriptions
                    .insert_no_overwrite(subscription.channel_id, subscription.id)
                    .is_err()
                {
                    if subscriptions.contains_left(&subscription.channel_id) {
                        self.send_warning(format!(
                            "Client is already subscribed to channel: {}; ignoring subscription",
                            subscription.channel_id
                        ));
                    } else {
                        assert!(subscriptions.contains_right(&subscription.id));
                        self.send_error(format!(
                            "Subscription ID was already used: {}; ignoring subscription",
                            subscription.id
                        ));
                    }
                    continue;
                }
            }

            tracing::debug!(
                "Client {} subscribed to channel {} with subscription id {}",
                self.addr,
                subscription.channel_id,
                subscription.id
            );
            if let Some(handler) = self.server_listener.as_ref() {
                handler.on_subscribe(
                    Client(self),
                    ChannelView {
                        id: channel.id,
                        topic: &channel.topic,
                    },
                );
            }
        }
    }

    fn on_get_parameters(
        &self,
        server: Arc<Server>,
        param_names: Vec<String>,
        request_id: Option<String>,
    ) {
        if !server.capabilities.contains(&Capability::Parameters) {
            self.send_error("Server does not support parameters capability".to_string());
            return;
        }

        if let Some(handler) = self.server_listener.as_ref() {
            let request_id = request_id.as_deref();
            let parameters = handler.on_get_parameters(Client(self), param_names, request_id);
            let message = protocol::server::parameters_json(&parameters, request_id);
            let _ = self.control_plane_tx.try_send(Message::text(message));
        }
    }

    fn on_set_parameters(
        &self,
        server: Arc<Server>,
        parameters: Vec<Parameter>,
        request_id: Option<String>,
    ) {
        if !server.capabilities.contains(&Capability::Parameters) {
            self.send_error("Server does not support parameters capability".to_string());
            return;
        }

        let updated_parameters = if let Some(handler) = self.server_listener.as_ref() {
            let request_id = request_id.as_deref();
            let updated_parameters =
                handler.on_set_parameters(Client(self), parameters, request_id);
            // Send all the updated_parameters back to the client if request_id is provided.
            // This is the behavior of the reference Python server implementation.
            if request_id.is_some() {
                let message = protocol::server::parameters_json(&updated_parameters, request_id);
                self.send_control_msg(Message::text(message));
            }
            updated_parameters
        } else {
            // This differs from the Python legacy ws-protocol implementation in that here we notify
            // subscribers about the parameters even if there's no ServerListener configured.
            // This seems to be a more sensible default.
            parameters
        };
        server.publish_parameter_values(updated_parameters);
    }

    fn update_parameters(&self, parameters: &[Parameter]) {
        // Hold the lock for as short a time as possible
        let subscribed_parameters: Vec<Parameter> = {
            let subscribed_parameters = self.parameter_subscriptions.lock();
            // Filter parameters to only send the ones the client is subscribed to
            parameters
                .iter()
                .filter(|p| subscribed_parameters.contains(&p.name))
                .cloned()
                .collect()
        };
        if subscribed_parameters.is_empty() {
            return;
        }
        let message = protocol::server::parameters_json(&subscribed_parameters, None);
        self.send_control_msg(Message::text(message));
    }

    fn on_parameters_subscribe(&self, server: Arc<Server>, param_names: Vec<String>) {
        if !server
            .capabilities
            .contains(&Capability::ParametersSubscribe)
        {
            self.send_error("Server does not support parametersSubscribe capability".to_string());
            return;
        }

        // We hold the server lock here the entire time to serialize
        // calls to subscribe and unsubscribe, otherwise there are all
        // kinds of race conditions here where handlers get invoked in
        // an order different than the order the events were applied,
        // leading to the listener thinking it has no subscribers to a
        // parameter when it actually does or visa versa.
        let mut new_param_subscriptions = Vec::with_capacity(
            self.server_listener
                .as_ref()
                .map(|_| param_names.len())
                .unwrap_or_default(),
        );
        let mut all_subscriptions = server.subscribed_parameters.lock();

        // Get the list of which subscriptions are new to the server (first time subscriptions)
        if self.server_listener.is_some() {
            for name in param_names.iter() {
                if all_subscriptions.insert(name.clone()) {
                    new_param_subscriptions.push(name.clone());
                }
            }
        }

        {
            // Track the client's own subscriptions
            let mut client_subscriptions = self.parameter_subscriptions.lock();
            client_subscriptions.extend(param_names);
        }

        if new_param_subscriptions.is_empty() {
            return;
        }

        if let Some(handler) = self.server_listener.as_ref() {
            // We hold the server subscribed_parameters mutex across the call to the handler
            // to synchrnize with other
            handler.on_parameters_subscribe(new_param_subscriptions);
        }
    }

    fn on_parameters_unsubscribe(&self, server: Arc<Server>, mut param_names: Vec<String>) {
        if !server
            .capabilities
            .contains(&Capability::ParametersSubscribe)
        {
            self.send_error("Server does not support parametersSubscribe capability".to_string());
            return;
        }

        // Like in subscribe, we first take the server-wide lock.
        let mut all_subscriptions = server.subscribed_parameters.lock();

        {
            // Remove the parameter subscriptions for this client,
            // and filter out any we weren't subscribed to.
            let mut client_subscriptions = self.parameter_subscriptions.lock();
            param_names.retain(|name| client_subscriptions.remove(name));
        }

        if param_names.is_empty() {
            // We didn't remove any subscriptions
            return;
        }

        let Some(handler) = self.server_listener.as_ref() else {
            return;
        };

        let mut unsubscribed_parameters = server.parameters_without_subscription(param_names);
        // Remove the unsubscribed parameters from the server's list of subscribed parameters
        unsubscribed_parameters.retain(|name| all_subscriptions.remove(name));
        // We have to hold the lock while calling the handler because we need
        // to synchronize this with other calls to on_parameters_subscribe and on_parameters_unsubscribe
        handler.on_parameters_unsubscribe(unsubscribed_parameters);
    }

    fn on_service_call(&self, req: protocol::client::ServiceCallRequest) {
        let Some(server) = self.server.upgrade() else {
            return;
        };

        // We have a response channel if and only if the server supports services.
        let service_id = req.service_id;
        let call_id = req.call_id;
        if !server.capabilities.contains(&Capability::Services) {
            self.send_service_call_failure(service_id, call_id, "Server does not support services");
            return;
        };

        // Lookup the requested service handler.
        let Some(service) = server.get_service(service_id) else {
            self.send_service_call_failure(service_id, call_id, "Unknown service");
            return;
        };

        // If this service declared a request encoding, ensure that it matches. Otherwise, ensure
        // that the request encoding is in the server's global list of supported encodings.
        if !service
            .request_encoding()
            .map(|e| e == req.encoding)
            .unwrap_or_else(|| server.supported_encodings.contains(&req.encoding))
        {
            self.send_service_call_failure(service_id, call_id, "Unsupported encoding");
            return;
        }

        // Acquire the semaphore, or reject if there are too many concurrenct requests.
        let Some(guard) = self.service_call_sem.try_acquire() else {
            self.send_service_call_failure(service_id, call_id, "Too many requests");
            return;
        };

        // Prepare the responder and the request.
        let responder = service::Responder::new(
            self.arc(),
            service.id(),
            call_id,
            service.response_encoding().unwrap_or(&req.encoding),
            guard,
        );
        let request = service::Request::new(service.clone(), call_id, req.encoding, req.payload);

        // Invoke the handler.
        service.call(Client(self), request, responder);
    }

    /// Sends a service call failure message to the client with the provided message.
    fn send_service_call_failure(&self, service_id: ServiceId, call_id: CallId, message: &str) {
        let msg = Message::text(protocol::server::service_call_failure(
            service_id, call_id, message,
        ));
        self.send_control_msg(msg);
    }

    /// Send an ad hoc error status message to the client, with the given message.
    fn send_error(&self, message: String) {
        self.send_status(Status::new(StatusLevel::Error, message));
    }

    /// Send an ad hoc warning status message to the client, with the given message.
    #[allow(dead_code)]
    fn send_warning(&self, message: String) {
        self.send_status(Status::new(StatusLevel::Warning, message));
    }

    /// Send a status message to the client.
    fn send_status(&self, status: Status) {
        let message = Message::text(serde_json::to_string(&status).unwrap());
        match status.level {
            StatusLevel::Info => {
                self.send_data_lossy(message, MAX_SEND_RETRIES);
            }
            _ => {
                self.send_control_msg(message);
            }
        }
    }
}

impl std::fmt::Debug for ConnectedClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("id", &self.id)
            .field("address", &self.addr)
            .finish()
    }
}

// A websocket server that implements the Foxglove WebSocket Protocol
impl Server {
    /// Generate a random session ID
    pub(crate) fn generate_session_id() -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|d| d.as_millis().to_string())
            .unwrap_or_default()
    }

    pub fn new(weak_self: Weak<Self>, opts: ServerOptions) -> Self {
        let mut capabilities = opts.capabilities.unwrap_or_default();
        let mut supported_encodings = opts.supported_encodings.unwrap_or_default();

        // If the server was declared with services, automatically add the "services" capability
        // and the set of supported request encodings.
        if !opts.services.is_empty() {
            capabilities.insert(Capability::Services);
            supported_encodings.extend(
                opts.services
                    .values()
                    .flat_map(|svc| svc.schema().request().map(|s| s.encoding.clone())),
            );
        }

        Server {
            weak_self,
            started: AtomicBool::new(false),
            message_backlog_size: opts
                .message_backlog_size
                .unwrap_or(DEFAULT_MESSAGE_BACKLOG_SIZE) as u32,
            runtime: opts.runtime.unwrap_or_else(get_runtime_handle),
            listener: opts.listener,
            session_id: parking_lot::RwLock::new(
                opts.session_id.unwrap_or_else(Self::generate_session_id),
            ),
            name: opts.name.unwrap_or_default(),
            clients: CowVec::new(),
            channels: parking_lot::RwLock::new(HashMap::new()),
            subscribed_parameters: parking_lot::Mutex::new(HashSet::new()),
            capabilities,
            supported_encodings,
            cancellation_token: CancellationToken::new(),
            services: parking_lot::RwLock::new(
                opts.services
                    .into_values()
                    .map(|s| (s.id(), Arc::new(s)))
                    .collect(),
            ),
        }
    }

    pub fn arc(&self) -> Arc<Self> {
        self.weak_self
            .upgrade()
            .expect("server cannot be dropped while in use")
    }

    // Returns a handle to the async runtime that this server is using.
    pub fn runtime(&self) -> &Handle {
        &self.runtime
    }

    // Spawn a task to accept all incoming connections and return
    pub async fn start(&self, host: &str, port: u16) -> Result<String, FoxgloveError> {
        if self.started.load(Acquire) {
            return Err(FoxgloveError::ServerAlreadyStarted);
        }
        let already_started = self.started.swap(true, AcqRel);
        assert!(!already_started);

        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(FoxgloveError::Bind)?;
        let bound_addr = listener
            .local_addr()
            .map_err(|err| FoxgloveError::Unspecified(err.into()))?
            .to_string();

        let cancellation_token = self.cancellation_token.clone();
        let server = self.arc().clone();
        self.runtime.spawn(async move {
            tokio::select! {
                () = handle_connections(server, listener) => (),
                () = cancellation_token.cancelled() => {
                    tracing::debug!("Closed connection handler");
                }
            }
        });

        tracing::info!("Started server on {}", bound_addr);

        Ok(bound_addr)
    }

    pub async fn stop(&self) {
        if self
            .started
            .compare_exchange(true, false, AcqRel, Acquire)
            .is_err()
        {
            return;
        }
        tracing::info!("Shutting down");
        let clients = self.clients.get();
        for client in clients.iter() {
            let mut sender = client.sender.lock().await;
            sender.send(Message::Close(None)).await.ok();
        }
        self.clients.clear();
        self.cancellation_token.cancel();
    }

    async fn advertise_channel(&self, channel: Arc<Channel>) {
        if channel.schema.is_none() {
            tracing::error!(
                "Ignoring advertise channel for {} because a schema is required",
                channel.topic
            );
            return;
        }

        self.channels.write().insert(channel.id, channel.clone());

        let message = match protocol::server::advertisement(&channel) {
            Ok(message) => message,
            Err(err) => {
                tracing::error!("Error creating advertise channel message to client: {err}");
                return;
            }
        };

        let clients = self.clients.get();
        for client in clients.iter() {
            if client.send_control_msg(Message::text(message.clone())) {
                tracing::debug!(
                    "Advertised channel {} with id {} to client {}",
                    channel.topic,
                    channel.id,
                    client.addr
                );
            }
        }
    }

    async fn unadvertise_channel(&self, channel_id: ChannelId) {
        self.channels.write().remove(&channel_id);

        let message = protocol::server::unadvertise(channel_id);
        let clients = self.clients.get();
        for client in clients.iter() {
            if client.send_control_msg(Message::text(message.clone())) {
                tracing::debug!(
                    "Unadvertised channel with id {} to client {}",
                    channel_id,
                    client.addr
                );
            }
        }
    }

    /// Filter param_names to just those with no subscribers
    fn parameters_without_subscription(&self, mut param_names: Vec<String>) -> Vec<String> {
        let clients = self.clients.get();
        for client in clients.iter() {
            let subscribed_parameters = client.parameter_subscriptions.lock();
            // Remove any parameters that are already subscribed to by this client
            param_names.retain(|name| !subscribed_parameters.contains(name));
        }
        // The remaining parameters are those with no subscribers
        param_names
    }

    /// Publish the current timestamp to all clients.
    #[cfg(feature = "unstable")]
    pub async fn broadcast_time(&self, timestamp_nanos: u64) {
        if !self.capabilities.contains(&Capability::Time) {
            tracing::error!("Server does not support time capability");
            return;
        }

        // https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#time
        let mut buf = BytesMut::with_capacity(9);
        buf.put_u8(protocol::server::BinaryOpcode::TimeData as u8);
        buf.put_u64_le(timestamp_nanos);
        let message = Message::binary(buf);

        let clients = self.clients.get();
        for client in clients.iter() {
            client.send_control_msg(message.clone());
        }
    }

    /// Publish parameter values to all clients.
    pub fn publish_parameter_values(&self, parameters: Vec<Parameter>) {
        if !self.capabilities.contains(&Capability::Parameters) {
            tracing::error!("Server does not support parameters capability");
            return;
        }

        let clients = self.clients.get();
        for client in clients.iter() {
            client.update_parameters(&parameters);
        }
    }

    /// Send a status message to all clients.
    pub fn publish_status(&self, status: Status) {
        let clients = self.clients.get();
        for client in clients.iter() {
            client.send_status(status.clone());
        }
    }

    /// Remove status messages by id from all clients.
    pub fn remove_status(&self, status_ids: Vec<String>) {
        let remove = protocol::server::RemoveStatus { status_ids };
        let message = Message::text(serde_json::to_string(&remove).unwrap());
        let clients = self.clients.get();
        for client in clients.iter() {
            client.send_control_msg(message.clone());
        }
    }

    /// Sets a new session ID and notifies all clients, causing them to reset their state.
    /// If no session ID is provided, generates a new one based on the current timestamp.
    pub fn clear_session(&self, new_session_id: Option<String>) {
        *self.session_id.write() = new_session_id.unwrap_or_else(Self::generate_session_id);

        let info_message = protocol::server::server_info(
            &self.session_id.read(),
            &self.name,
            &self.capabilities,
            &self.supported_encodings,
        );

        let message = Message::text(info_message);
        let clients = self.clients.get();
        for client in clients.iter() {
            client.send_control_msg(message.clone());
        }
    }

    /// When a new client connects:
    /// - Handshake
    /// - Send ServerInfo
    /// - Advertise existing channels
    /// - Advertise existing services
    /// - Listen for client meesages
    async fn handle_connection(self: Arc<Self>, stream: TcpStream, addr: SocketAddr) {
        let ws_stream = match do_handshake(stream).await {
            Ok(ws_stream) => ws_stream,
            Err(_) => {
                tracing::error!("Dropping client {addr}: {}", WSError::HandshakeError);
                return;
            }
        };

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let info_message = protocol::server::server_info(
            &self.session_id.read(),
            &self.name,
            &self.capabilities,
            &self.supported_encodings,
        );
        if let Err(err) = ws_sender.send(Message::text(info_message)).await {
            // ServerInfo is required; do not store this client.
            tracing::error!("Failed to send required server info: {err}");
            return;
        }

        static CLIENT_ID: AtomicU32 = AtomicU32::new(1);
        let id = ClientId(CLIENT_ID.fetch_add(1, Relaxed));

        let (data_tx, data_rx) = flume::bounded(self.message_backlog_size as usize);
        let (ctrl_tx, ctrl_rx) = flume::bounded(DEFAULT_CONTROL_PLANE_BACKLOG_SIZE);

        let new_client = Arc::new_cyclic(|weak_self| ConnectedClient {
            id,
            addr,
            weak_self: weak_self.clone(),
            sender: Mutex::new(ws_sender),
            data_plane_tx: data_tx,
            data_plane_rx: data_rx,
            control_plane_tx: ctrl_tx,
            control_plane_rx: ctrl_rx,
            service_call_sem: service::Semaphore::new(DEFAULT_SERVICE_CALLS_PER_CLIENT),
            subscriptions: parking_lot::Mutex::new(BiHashMap::new()),
            advertised_channels: parking_lot::Mutex::new(HashMap::new()),
            parameter_subscriptions: parking_lot::Mutex::new(HashSet::new()),
            server_listener: self.listener.clone(),
            server: self.weak_self.clone(),
        });

        self.register_client_and_advertise(new_client.clone()).await;

        let receive_messages = async {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Close(_)) => {
                        tracing::info!("Connection closed by client {addr}");
                        // Finish receive_messages
                        return;
                    }
                    Ok(msg) => {
                        new_client.handle_message(msg);
                    }
                    Err(err) => {
                        tracing::error!("Error receiving from client {addr}: {err}");
                    }
                }
            }
        };

        let send_control_messages = async {
            while let Ok(msg) = new_client.control_plane_rx.recv_async().await {
                let mut sender = new_client.sender.lock().await;
                if let Err(err) = sender.send(msg).await {
                    if self.started.load(Acquire) {
                        tracing::error!("Error sending control message to client {addr}: {err}");
                    } else {
                        new_client.control_plane_rx.drain();
                        new_client.data_plane_rx.drain();
                    }
                }
            }
        };

        // send_messages forwards messages from the rx size of the data plane to the sender
        let send_messages = async {
            while let Ok(msg) = new_client.data_plane_rx.recv_async().await {
                let mut sender = new_client.sender.lock().await;
                if let Err(err) = sender.send(msg).await {
                    if self.started.load(Acquire) {
                        tracing::error!("Error sending data message to client {addr}: {err}");
                    } else {
                        new_client.control_plane_rx.drain();
                        new_client.data_plane_rx.drain();
                    }
                }
            }
        };

        // Run send and receive loops concurrently, and wait for receive to complete
        tokio::select! {
            _ = receive_messages => {
                tracing::debug!("Receive messages task completed");
            }
            _ = send_control_messages => {
                tracing::error!("Send control messages task completed");
            }
            _ = send_messages => {
                tracing::error!("Send messages task completed");
            }
        }

        self.clients.retain(|c| !Arc::ptr_eq(c, &new_client));
        new_client.on_disconnect(&self);
    }

    async fn register_client_and_advertise(&self, client: Arc<ConnectedClient>) {
        // Lock the sender so the channel advertisement is the first message sent
        let mut sender = client.sender.lock().await;
        // Add the client to self.clients
        self.clients.push(client.clone());

        // Advertise existing channels to the new client. We must do this AFTER adding the client to clients,
        // otherwise there is potential for the client to miss a new channel advertisement.
        // Create a copy of the channels to avoid holding the lock while sending messages.
        let channels: Vec<_> = self.channels.read().values().cloned().collect();
        let services: Vec<_> = self.services.read().values().cloned().collect();

        tracing::info!(
            "Registered client {}; advertising {} channels and {} services",
            client.addr,
            channels.len(),
            services.len(),
        );

        for channel in channels.into_iter() {
            let message = match protocol::server::advertisement(&channel) {
                Ok(message) => message,
                Err(err) => {
                    tracing::error!("Error creating advertise channel message to client: {err}");
                    return;
                }
            };

            if let Err(err) = sender.send(Message::text(message)).await {
                // We can't send messages to the client. Maybe we can still receive messages? Let's continue.
                tracing::error!("Error advertising channel: {err}");
                break;
            }

            tracing::debug!(
                "Advertised channel {} with id {} to client {}",
                channel.topic,
                channel.id,
                client.addr
            );
        }

        if !services.is_empty() {
            let msg = Message::text(protocol::server::advertise_services(
                services.iter().map(|s| s.as_ref()),
            ));
            if let Err(err) = sender.send(msg).await {
                tracing::error!("Error advertising services: {err}");
            } else {
                for service in services {
                    tracing::debug!(
                        "Advertised service {} with id {} to client {}",
                        service.name(),
                        service.id(),
                        client.addr
                    );
                }
            }
        }
    }

    /// Adds new services, and advertises them to all clients.
    ///
    /// This method will fail if the services capability was not declared, or if a service name is
    /// not unique.
    pub fn add_services(&self, new_services: Vec<Service>) -> Result<(), FoxgloveError> {
        // Make sure that the server supports services.
        if !self.capabilities.contains(&Capability::Services) {
            return Err(FoxgloveError::ServicesNotSupported);
        }

        let mut new_names = HashMap::with_capacity(new_services.len());
        for service in &new_services {
            // Ensure that the new service names are unique.
            if new_names
                .insert(service.name().to_string(), service.id())
                .is_some()
            {
                return Err(FoxgloveError::DuplicateService(service.name().to_string()));
            }

            // If the service doesn't declare a request encoding, there must be at least one
            // encoding declared in the global list.
            if service.request_encoding().is_none() && self.supported_encodings.is_empty() {
                return Err(FoxgloveError::MissingRequestEncoding(
                    service.name().to_string(),
                ));
            }
        }

        // Prepare an advertisement.
        let msg = Message::text(protocol::server::advertise_services(&new_services));

        {
            // Ensure that the new service names are not already registered.
            let mut services = self.services.write();
            for service in services.values() {
                if new_names.contains_key(service.name()) {
                    return Err(FoxgloveError::DuplicateService(service.name().to_string()));
                }
            }

            // Update the service map.
            for service in new_services {
                services.insert(service.id(), Arc::new(service));
            }
        }

        // Send advertisements.
        let clients = self.clients.get();
        for client in clients.iter().cloned() {
            for (name, id) in new_names.iter() {
                tracing::debug!(
                    "Advertising service {name} with id {id} to client {}",
                    client.addr
                );
            }
            client.send_control_msg(msg.clone());
        }

        Ok(())
    }

    /// Removes services, and unadvertises them to all clients.
    ///
    /// Unrecognized service IDs are silently ignored.
    pub fn remove_services(&self, ids: &[ServiceId]) {
        // Remove services from the map.
        let mut old_services = HashMap::with_capacity(ids.len());
        {
            let mut services = self.services.write();
            for id in ids {
                if let Some(service) = services.remove(id) {
                    old_services.insert(*id, service.name().to_string());
                }
            }
        }
        if old_services.is_empty() {
            return;
        }

        // Prepare an unadvertisement.
        let msg = Message::text(protocol::server::unadvertise_services(
            &old_services.keys().cloned().collect::<Vec<_>>(),
        ));

        let clients = self.clients.get();
        for client in clients.iter().cloned() {
            for (id, name) in old_services.iter() {
                tracing::debug!(
                    "Unadvertising service {name} with id {id} to client {}",
                    client.addr
                );
            }
            client.send_control_msg(msg.clone());
        }
    }

    // Looks up a service by ID.
    fn get_service(&self, id: ServiceId) -> Option<Arc<Service>> {
        self.services.read().get(&id).cloned()
    }
}

#[derive(Debug, Clone, Copy)]
enum SendLossyResult {
    Sent,
    #[allow(dead_code)]
    SentLossy(usize),
    ExhaustedRetries,
}

/// Attempt to send a message on the channel.
///
/// If the channel is non-full, this function returns `SendLossyResult::Sent`.
///
/// If the channel is full, drop the oldest message and try again. If the send eventually succeeds
/// in this manner, this function returns `SendLossyResult::SentLossy(dropped)`. If the maximum
/// number of retries is reached, it returns `SendLossyResult::ExhaustedRetries`.
fn send_lossy(
    client_addr: &SocketAddr,
    tx: &flume::Sender<Message>,
    rx: &flume::Receiver<Message>,
    mut message: Message,
    retries: usize,
) -> SendLossyResult {
    // If the queue is full, drop the oldest message(s). We do this because the websocket
    // client is falling behind, and we either start dropping messages, or we'll end up
    // buffering until we run out of memory. There's no point in that because the client is
    // unlikely to catch up and be able to consume the messages.
    let mut dropped = 0;
    loop {
        match (dropped, tx.try_send(message)) {
            (0, Ok(_)) => return SendLossyResult::Sent,
            (dropped, Ok(_)) => {
                tracing::warn!(
                    "outbox for client {} full, dropped {dropped} messages",
                    client_addr
                );
                return SendLossyResult::SentLossy(dropped);
            }
            (_, Err(TrySendError::Disconnected(_))) => unreachable!("we're holding rx"),
            (_, Err(TrySendError::Full(rejected))) => {
                if dropped >= retries {
                    tracing::warn!(
                        "outbox for client {} full, dropping message after 10 attempts",
                        client_addr
                    );
                    return SendLossyResult::ExhaustedRetries;
                }
                message = rejected;
                let _ = rx.try_recv();
                dropped += 1
            }
        }
    }
}

impl LogSink for Server {
    fn log(
        &self,
        channel: &Arc<Channel>,
        msg: &[u8],
        metadata: &Metadata,
    ) -> Result<(), FoxgloveError> {
        let clients = self.clients.get();
        for client in clients.iter() {
            let subscriptions = client.subscriptions.lock();
            let Some(subscription_id) = subscriptions.get_by_left(&channel.id).cloned() else {
                continue;
            };

            // https://github.com/foxglove/ws-protocol/blob/main/docs/spec.md#message-data
            let header_size: usize = 1 + 4 + 8;
            let mut buf = BytesMut::with_capacity(header_size + msg.len());
            buf.put_u8(protocol::server::BinaryOpcode::MessageData as u8);
            buf.put_u32_le(subscription_id.into());
            buf.put_u64_le(metadata.log_time);
            buf.put_slice(msg);

            let message = Message::binary(buf);

            client.send_data_lossy(message, MAX_SEND_RETRIES);
        }
        Ok(())
    }

    /// Server has an available channel. Advertise to all clients.
    fn add_channel(&self, channel: &Arc<Channel>) {
        let server = self.arc();
        let ch = channel.clone();
        self.runtime
            .spawn(async move { server.advertise_channel(ch).await });
    }

    /// A channel is being removed. Unadvertise to all clients.
    fn remove_channel(&self, channel: &Channel) {
        let server = self.arc();
        let channel_id = channel.id();
        self.runtime
            .spawn(async move { server.unadvertise_channel(channel_id).await });
    }
}

pub(crate) fn create_server(opts: ServerOptions) -> Arc<Server> {
    Arc::new_cyclic(|weak_self| Server::new(weak_self.clone(), opts))
}

// Spawn a new task for each incoming connection
async fn handle_connections(server: Arc<Server>, listener: TcpListener) {
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(server.clone().handle_connection(stream, addr));
    }
}

/// Add the subprotocol header to the response if the client requested one we support.
/// If the client doesn't support our protocol, do not include the protocol header in the response;
/// the client must fail the connection. [WebSocket RFC](https://www.rfc-editor.org/rfc/rfc6455#section-4)
async fn do_handshake(stream: TcpStream) -> Result<WebSocketStream<TcpStream>, tungstenite::Error> {
    tokio_tungstenite::accept_hdr_async(
        stream,
        |req: &server::Request, mut res: server::Response| {
            let all_headers = req.headers().get_all("sec-websocket-protocol");
            if all_headers.iter().any(|h| {
                (*h).to_str()
                    .unwrap_or_default()
                    .split(',')
                    .any(|s| s.trim() == SUBPROTOCOL)
            }) {
                res.headers_mut().insert(
                    "sec-websocket-protocol",
                    HeaderValue::from_static(SUBPROTOCOL),
                );
            };
            Ok(res)
        },
    )
    .await
}
