//! Test utilities.

mod log_context;
mod log_sink;

use crate::channel::ChannelId;
use crate::websocket::{
    AssetResponder, ChannelView, Client, ClientChannelId, ClientChannelView, ClientId, Parameter,
    ServerListener,
};
pub use log_context::GlobalContextTest;
pub use log_sink::{ErrorSink, MockSink, RecordingSink};
use parking_lot::Mutex;

#[allow(dead_code)]
pub(crate) struct ClientChannelInfo {
    pub(crate) id: ClientChannelId,
    pub(crate) topic: String,
}

impl From<ClientChannelView<'_>> for ClientChannelInfo {
    fn from(channel: ClientChannelView) -> Self {
        Self {
            id: channel.id(),
            topic: channel.topic().to_string(),
        }
    }
}

pub(crate) struct ChannelInfo {
    pub(crate) id: ChannelId,
    pub(crate) topic: String,
}

impl From<ChannelView<'_>> for ChannelInfo {
    fn from(channel: ChannelView) -> Self {
        Self {
            id: channel.id(),
            topic: channel.topic().to_string(),
        }
    }
}

pub(crate) struct MessageData {
    #[allow(dead_code)]
    pub client_id: ClientId,
    pub channel: ClientChannelInfo,
    pub data: Vec<u8>,
}

pub(crate) struct GetParameters {
    #[allow(dead_code)]
    pub client_id: ClientId,
    pub param_names: Vec<String>,
    pub request_id: Option<String>,
}

pub(crate) struct SetParameters {
    #[allow(dead_code)]
    pub client_id: ClientId,
    pub parameters: Vec<Parameter>,
    pub request_id: Option<String>,
}

pub(crate) struct RecordingServerListener {
    message_data: Mutex<Vec<MessageData>>,
    subscribe: Mutex<Vec<(ClientId, ChannelInfo)>>,
    unsubscribe: Mutex<Vec<(ClientId, ChannelInfo)>>,
    client_advertise: Mutex<Vec<(ClientId, ClientChannelInfo)>>,
    client_unadvertise: Mutex<Vec<(ClientId, ClientChannelInfo)>>,
    parameters_subscribe: Mutex<Vec<Vec<String>>>,
    parameters_unsubscribe: Mutex<Vec<Vec<String>>>,
    parameters_get: Mutex<Vec<GetParameters>>,
    parameters_set: Mutex<Vec<SetParameters>>,
    parameters_get_result: Mutex<Vec<Parameter>>,
    fetch_asset: Mutex<Vec<String>>,
}

impl RecordingServerListener {
    pub fn new() -> Self {
        Self {
            message_data: Mutex::new(Vec::new()),
            subscribe: Mutex::new(Vec::new()),
            unsubscribe: Mutex::new(Vec::new()),
            client_advertise: Mutex::new(Vec::new()),
            client_unadvertise: Mutex::new(Vec::new()),
            parameters_subscribe: Mutex::new(Vec::new()),
            parameters_unsubscribe: Mutex::new(Vec::new()),
            parameters_get: Mutex::new(Vec::new()),
            parameters_set: Mutex::new(Vec::new()),
            parameters_get_result: Mutex::new(Vec::new()),
            fetch_asset: Mutex::new(Vec::new()),
        }
    }

    pub fn take_message_data(&self) -> Vec<MessageData> {
        std::mem::take(&mut self.message_data.lock())
    }

    pub fn take_subscribe(&self) -> Vec<(ClientId, ChannelInfo)> {
        std::mem::take(&mut self.subscribe.lock())
    }

    pub fn take_unsubscribe(&self) -> Vec<(ClientId, ChannelInfo)> {
        std::mem::take(&mut self.unsubscribe.lock())
    }

    pub fn take_client_advertise(&self) -> Vec<(ClientId, ClientChannelInfo)> {
        std::mem::take(&mut self.client_advertise.lock())
    }

    pub fn take_client_unadvertise(&self) -> Vec<(ClientId, ClientChannelInfo)> {
        std::mem::take(&mut self.client_unadvertise.lock())
    }

    pub fn take_parameters_subscribe(&self) -> Vec<Vec<String>> {
        std::mem::take(&mut self.parameters_subscribe.lock())
    }

    pub fn take_parameters_unsubscribe(&self) -> Vec<Vec<String>> {
        std::mem::take(&mut self.parameters_unsubscribe.lock())
    }

    pub fn take_parameters_get(&self) -> Vec<GetParameters> {
        std::mem::take(&mut self.parameters_get.lock())
    }

    pub fn set_parameters_get_result(&self, result: Vec<Parameter>) {
        *self.parameters_get_result.lock() = result;
    }

    pub fn take_parameters_set(&self) -> Vec<SetParameters> {
        std::mem::take(&mut self.parameters_set.lock())
    }

    pub fn take_fetch_asset(&self) -> Vec<String> {
        std::mem::take(&mut self.fetch_asset.lock())
    }
}

impl ServerListener for RecordingServerListener {
    fn on_message_data(&self, client: Client, channel: ClientChannelView, payload: &[u8]) {
        let mut data = self.message_data.lock();
        data.push(MessageData {
            client_id: client.id(),
            channel: channel.into(),
            data: payload.to_vec(),
        });
    }

    fn on_subscribe(&self, client: Client, channel: ChannelView) {
        let mut subs = self.subscribe.lock();
        subs.push((client.id(), channel.into()));
    }

    fn on_unsubscribe(&self, client: Client, channel: ChannelView) {
        let mut unsubs = self.unsubscribe.lock();
        unsubs.push((client.id(), channel.into()));
    }

    fn on_client_advertise(&self, client: Client, channel: ClientChannelView) {
        let mut adverts = self.client_advertise.lock();
        adverts.push((client.id(), channel.into()));
    }

    fn on_client_unadvertise(&self, client: Client, channel: ClientChannelView) {
        let mut unadverts = self.client_unadvertise.lock();
        unadverts.push((client.id(), channel.into()));
    }

    fn on_get_parameters(
        &self,
        client: Client,
        param_names: Vec<String>,
        request_id: Option<&str>,
    ) -> Vec<Parameter> {
        let mut gets = self.parameters_get.lock();
        gets.push(GetParameters {
            client_id: client.id(),
            param_names: param_names.clone(),
            request_id: request_id.map(|s| s.to_string()),
        });
        self.parameters_get_result.lock().clone()
    }

    fn on_set_parameters(
        &self,
        client: Client,
        parameters: Vec<Parameter>,
        request_id: Option<&str>,
    ) -> Vec<Parameter> {
        let mut sets = self.parameters_set.lock();
        sets.push(SetParameters {
            client_id: client.id(),
            parameters: parameters.clone(),
            request_id: request_id.map(|s| s.to_string()),
        });
        parameters
    }

    fn on_parameters_subscribe(&self, param_names: Vec<String>) {
        let mut subs = self.parameters_subscribe.lock();
        subs.push(param_names.clone());
    }

    fn on_parameters_unsubscribe(&self, param_names: Vec<String>) {
        let mut unsubs = self.parameters_unsubscribe.lock();
        unsubs.push(param_names.clone());
    }

    fn on_fetch_asset(&self, uri: String, responder: AssetResponder) {
        let mut fetches = self.fetch_asset.lock();
        let return_error = uri.ends_with("error");
        fetches.push(uri);
        if return_error {
            responder.send_error("test error");
        } else {
            responder.send_data(b"test data");
        }
    }
}
