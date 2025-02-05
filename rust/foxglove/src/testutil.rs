//! Test utilities.

mod log_context;
mod log_sink;

use crate::channel::ChannelId;
use crate::websocket::{ClientChannel, ClientChannelId, ServerListener};
pub use log_context::GlobalContextTest;
pub use log_sink::{ErrorSink, MockSink, RecordingSink};
use parking_lot::Mutex;

#[allow(dead_code)]
pub(crate) struct RecordingServerListener {
    message_data: Mutex<Vec<(ClientChannelId, Vec<u8>)>>,
    subscribe: Mutex<Vec<ChannelId>>,
    unsubscribe: Mutex<Vec<ChannelId>>,
    client_advertise: Mutex<Vec<ClientChannel>>,
    client_unadvertise: Mutex<Vec<ClientChannelId>>,
}

impl RecordingServerListener {
    #[allow(dead_code)]
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
    pub fn message_data(&self) -> Vec<(ClientChannelId, Vec<u8>)> {
        std::mem::take(&mut self.message_data.lock())
    }

    #[allow(dead_code)]
    pub fn subscribe(&self) -> Vec<ChannelId> {
        std::mem::take(&mut self.subscribe.lock())
    }

    #[allow(dead_code)]
    pub fn unsubscribe(&self) -> Vec<ChannelId> {
        std::mem::take(&mut self.unsubscribe.lock())
    }

    #[allow(dead_code)]
    pub fn client_advertise(&self) -> Vec<ClientChannel> {
        std::mem::take(&mut self.client_advertise.lock())
    }

    #[allow(dead_code)]
    pub fn client_unadvertise(&self) -> Vec<ClientChannelId> {
        std::mem::take(&mut self.client_unadvertise.lock())
    }
}

impl ServerListener for RecordingServerListener {
    fn on_message_data(&self, channel_id: ClientChannelId, payload: &[u8]) {
        let mut data = self.message_data.lock();
        data.push((channel_id, payload.to_vec()));
    }

    fn on_subscribe(&self, channel_id: ChannelId) {
        let mut subs = self.subscribe.lock();
        subs.push(channel_id);
    }

    fn on_unsubscribe(&self, channel_id: ChannelId) {
        let mut unsubs = self.unsubscribe.lock();
        unsubs.push(channel_id);
    }

    fn on_client_advertise(&self, channel: &ClientChannel) {
        let mut adverts = self.client_advertise.lock();
        adverts.push(channel.clone());
    }

    fn on_client_unadvertise(&self, channel_id: ClientChannelId) {
        let mut unadverts = self.client_unadvertise.lock();
        unadverts.push(channel_id);
    }
}
