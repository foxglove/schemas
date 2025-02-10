//! Test utilities.

mod log_context;
mod log_sink;

use crate::channel::ChannelId;
use crate::websocket::{
    ChannelView, Client, ClientChannelId, ClientChannelView, ClientId, ServerListener,
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

pub(crate) struct RecordingServerListener {
    message_data: Mutex<Vec<(ClientId, ClientChannelInfo, Vec<u8>)>>,
    subscribe: Mutex<Vec<(ClientId, ChannelInfo)>>,
    unsubscribe: Mutex<Vec<(ClientId, ChannelInfo)>>,
    client_advertise: Mutex<Vec<(ClientId, ClientChannelInfo)>>,
    client_unadvertise: Mutex<Vec<(ClientId, ClientChannelInfo)>>,
}

impl RecordingServerListener {
    pub fn new() -> Self {
        Self {
            message_data: Mutex::new(Vec::new()),
            subscribe: Mutex::new(Vec::new()),
            unsubscribe: Mutex::new(Vec::new()),
            client_advertise: Mutex::new(Vec::new()),
            client_unadvertise: Mutex::new(Vec::new()),
        }
    }

    #[allow(dead_code)]
    pub fn take_message_data(&self) -> Vec<(ClientId, ClientChannelInfo, Vec<u8>)> {
        std::mem::take(&mut self.message_data.lock())
    }

    pub fn take_subscribe(&self) -> Vec<(ClientId, ChannelInfo)> {
        std::mem::take(&mut self.subscribe.lock())
    }

    pub fn take_unsubscribe(&self) -> Vec<(ClientId, ChannelInfo)> {
        std::mem::take(&mut self.unsubscribe.lock())
    }

    #[allow(dead_code)]
    pub fn take_client_advertise(&self) -> Vec<(ClientId, ClientChannelInfo)> {
        std::mem::take(&mut self.client_advertise.lock())
    }

    #[allow(dead_code)]
    pub fn take_client_unadvertise(&self) -> Vec<(ClientId, ClientChannelInfo)> {
        std::mem::take(&mut self.client_unadvertise.lock())
    }
}

impl ServerListener for RecordingServerListener {
    fn on_message_data(&self, client: Client, channel: ClientChannelView, payload: &[u8]) {
        let mut data = self.message_data.lock();
        data.push((client.id(), channel.into(), payload.to_vec()));
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
}
