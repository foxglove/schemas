//! Test utilities.

mod log_context;
mod log_sink;

use crate::websocket::{ClientChannel, ClientChannelId, ClientId, ServerListener};
use crate::Channel;
pub use log_context::GlobalContextTest;
pub use log_sink::{ErrorSink, MockSink, RecordingSink};
use parking_lot::Mutex;
use std::sync::Arc;

pub(crate) struct RecordingServerListener {
    message_data: Mutex<Vec<(ClientId, ClientChannelId, Vec<u8>)>>,
    subscribe: Mutex<Vec<(ClientId, Arc<Channel>)>>,
    unsubscribe: Mutex<Vec<(ClientId, Arc<Channel>)>>,
    client_advertise: Mutex<Vec<(ClientId, ClientChannel)>>,
    client_unadvertise: Mutex<Vec<(ClientId, ClientChannelId)>>,
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
    pub fn take_message_data(&self) -> Vec<(ClientId, ClientChannelId, Vec<u8>)> {
        std::mem::take(&mut self.message_data.lock())
    }

    pub fn take_subscribe(&self) -> Vec<(ClientId, Arc<Channel>)> {
        std::mem::take(&mut self.subscribe.lock())
    }

    pub fn take_unsubscribe(&self) -> Vec<(ClientId, Arc<Channel>)> {
        std::mem::take(&mut self.unsubscribe.lock())
    }

    #[allow(dead_code)]
    pub fn take_client_advertise(&self) -> Vec<(ClientId, ClientChannel)> {
        std::mem::take(&mut self.client_advertise.lock())
    }

    #[allow(dead_code)]
    pub fn take_client_unadvertise(&self) -> Vec<(ClientId, ClientChannelId)> {
        std::mem::take(&mut self.client_unadvertise.lock())
    }
}

impl ServerListener for RecordingServerListener {
    fn on_message_data(&self, client_id: ClientId, channel_id: ClientChannelId, payload: &[u8]) {
        let mut data = self.message_data.lock();
        data.push((client_id, channel_id, payload.to_vec()));
    }

    fn on_subscribe(&self, client_id: ClientId, channel: Arc<Channel>) {
        let mut subs = self.subscribe.lock();
        subs.push((client_id, channel));
    }

    fn on_unsubscribe(&self, client_id: ClientId, channel: Arc<Channel>) {
        let mut unsubs = self.unsubscribe.lock();
        unsubs.push((client_id, channel));
    }

    fn on_client_advertise(&self, client_id: ClientId, channel: &ClientChannel) {
        let mut adverts = self.client_advertise.lock();
        adverts.push((client_id, channel.clone()));
    }

    fn on_client_unadvertise(&self, client_id: ClientId, channel_id: ClientChannelId) {
        let mut unadverts = self.client_unadvertise.lock();
        unadverts.push((client_id, channel_id));
    }
}
