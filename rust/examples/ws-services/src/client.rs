use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::types::{IntBinRequest, IntBinResponse, SetBoolRequest, SetBoolResponse};
use crate::Config;
use anyhow::{anyhow, Context, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::Deserialize;
use tokio::{
    sync::oneshot,
    task::{JoinHandle, JoinSet},
};
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue, Message};
use tracing::{error, info};

pub(crate) const SDK_SUBPROTOCOL: HeaderValue = HeaderValue::from_static("foxglove.sdk.v1");

enum ServerMessage {
    ServerInfo,
    AdvertiseServices(AdvertiseServices),
    UnadvertiseServices(UnadvertiseServices),
    ServiceCallResponse(ServiceCallResponse),
    ServiceCallFailure(ServiceCallFailure),
}
impl ServerMessage {
    pub fn parse_message(message: Message) -> Result<Option<Self>> {
        match message {
            Message::Text(bytes) => Self::parse_json(bytes.as_str()).map(Some),
            Message::Binary(bytes) => Self::parse_binary(bytes),
            _ => Err(anyhow!("unhandled message {message:?}")),
        }
    }

    fn parse_json(json: &str) -> Result<Self> {
        let msg = serde_json::from_str::<JsonMessage>(json)?;
        Ok(Self::from(msg))
    }

    fn parse_binary(mut data: Bytes) -> Result<Option<Self>> {
        if data.is_empty() {
            Ok(None)
        } else {
            let opcode = data.get_u8();
            match BinaryOpcode::from_repr(opcode) {
                Some(BinaryOpcode::ServiceCallResponse) => ServiceCallResponse::parse(data)
                    .map(ServerMessage::ServiceCallResponse)
                    .map(Some),
                _ => Err(anyhow!("invalid opcode {opcode}")),
            }
        }
    }
}

#[repr(u8)]
#[derive(strum::FromRepr)]
enum BinaryOpcode {
    ServiceCallRequest = 2,
    ServiceCallResponse = 3,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "op")]
enum JsonMessage {
    ServerInfo,
    AdvertiseServices(AdvertiseServices),
    UnadvertiseServices(UnadvertiseServices),
    ServiceCallFailure(ServiceCallFailure),
}

impl From<JsonMessage> for ServerMessage {
    fn from(m: JsonMessage) -> Self {
        match m {
            JsonMessage::ServerInfo => ServerMessage::ServerInfo,
            JsonMessage::AdvertiseServices(m) => ServerMessage::AdvertiseServices(m),
            JsonMessage::UnadvertiseServices(m) => ServerMessage::UnadvertiseServices(m),
            JsonMessage::ServiceCallFailure(m) => ServerMessage::ServiceCallFailure(m),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdvertiseServices {
    services: Vec<AdvertiseService>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdvertiseService {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnadvertiseServices {
    service_ids: Vec<u32>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ServiceCallResponse {
    service_id: u32,
    call_id: u32,
    encoding: String,
    payload: Bytes,
}
impl ServiceCallResponse {
    fn parse(mut data: Bytes) -> Result<Self> {
        // 4-byte service id, call id, and encoding length.
        if data.remaining() < 12 {
            return Err(anyhow!("Buffer too short"));
        }
        let service_id = data.get_u32_le();
        let call_id = data.get_u32_le();
        let encoding_length = data.get_u32_le() as usize;
        if data.remaining() < encoding_length {
            return Err(anyhow!("Buffer too short"));
        }
        let encoding = std::str::from_utf8(&data[..encoding_length])?.to_string();
        data.advance(encoding_length);
        Ok(Self {
            service_id,
            call_id,
            encoding,
            payload: data,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceCallFailure {
    service_id: u32,
    call_id: u32,
    message: String,
}

type Stream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

#[derive(Default)]
struct Inner {
    services: HashMap<String, u32>,
    service_calls: HashMap<u32, oneshot::Sender<Result<Bytes>>>,
}

struct ServiceCall {
    msg: Message,
    rx: oneshot::Receiver<Result<Bytes>>,
}

#[derive(Default, Clone)]
struct State(Arc<parking_lot::RwLock<Inner>>);
impl State {
    fn on_advertise_services(&self, msg: AdvertiseServices) {
        let mut inner = self.0.write();
        for service in msg.services {
            let name = service.name;
            let id = service.id;
            if let Some(prev_id) = inner.services.insert(name.clone(), id) {
                info!("Updated service {} id ({} -> {})", name, prev_id, id);
            } else {
                info!("Added service {} ({})", name, id);
            }
        }
    }

    fn on_unadvertise_services(&self, msg: UnadvertiseServices) {
        let mut inner = self.0.write();
        let ids: HashSet<_> = msg.service_ids.into_iter().collect();
        inner.services.retain(|name, id| {
            if ids.contains(id) {
                info!("Removed service {} ({})", name, id);
                false
            } else {
                true
            }
        });
    }

    fn on_service_call_response(&self, msg: ServiceCallResponse) {
        self.complete_service_call(msg.call_id, Ok(msg.payload));
    }

    fn on_service_call_failure(&self, msg: ServiceCallFailure) {
        self.complete_service_call(msg.call_id, Err(anyhow!(msg.message)));
    }

    fn get_service_id(&self, name: &str) -> Option<u32> {
        self.0.read().services.get(name).copied()
    }

    fn service_call(&self, service_id: u32, encoding: &str, payload: Bytes) -> ServiceCall {
        static CALL_ID: AtomicU32 = AtomicU32::new(0);
        let call_id = CALL_ID.fetch_add(1, Ordering::Relaxed);
        let encoding_raw = encoding.as_bytes();
        let mut buf = BytesMut::new();
        buf.put_u8(BinaryOpcode::ServiceCallRequest as u8);
        buf.put_u32_le(service_id);
        buf.put_u32_le(call_id);
        buf.put_u32_le(encoding_raw.len() as u32);
        buf.put(encoding_raw);
        buf.put(payload);
        let (tx, rx) = oneshot::channel();
        let prev = self.0.write().service_calls.insert(call_id, tx);
        assert!(prev.is_none());
        ServiceCall {
            msg: Message::Binary(buf.into()),
            rx,
        }
    }

    fn complete_service_call(&self, call_id: u32, result: Result<Bytes>) {
        let mut inner = self.0.write();
        if let Some(tx) = inner.service_calls.remove(&call_id) {
            let _ = tx.send(result);
        } else {
            error!("unexpected callback for {call_id}");
        }
    }
}

async fn rx_task(mut rx: SplitStream<Stream>, state: State) {
    while let Some(msg) = rx.next().await {
        let msg = msg.expect("Failed to receive message");
        let msg = match ServerMessage::parse_message(msg) {
            Ok(msg) => msg,
            Err(err) => {
                error!("Failed to parse message: {err}");
                continue;
            }
        };
        let Some(msg) = msg else {
            continue;
        };
        match msg {
            ServerMessage::ServerInfo => (),
            ServerMessage::AdvertiseServices(msg) => state.on_advertise_services(msg),
            ServerMessage::UnadvertiseServices(msg) => state.on_unadvertise_services(msg),
            ServerMessage::ServiceCallResponse(msg) => state.on_service_call_response(msg),
            ServerMessage::ServiceCallFailure(msg) => state.on_service_call_failure(msg),
        }
    }
}

struct Client {
    tx: tokio::sync::Mutex<SplitSink<Stream, Message>>,
    state: State,
    rx_task: JoinHandle<()>,
}
impl Drop for Client {
    fn drop(&mut self) {
        self.rx_task.abort();
    }
}
impl Client {
    async fn connect(host: &str, port: u16) -> Result<Self> {
        let mut request = format!("ws://{host}:{port}/")
            .into_client_request()
            .context("Failed to build request")?;

        request
            .headers_mut()
            .insert("sec-websocket-protocol", SDK_SUBPROTOCOL);

        let (stream, response) = tokio_tungstenite::connect_async(request)
            .await
            .context("Failed to connect")?;

        let Some(protocol) = response.headers().get("sec-websocket-protocol") else {
            return Err(anyhow!("Missing subprotocol from server"));
        };

        if protocol != SDK_SUBPROTOCOL {
            return Err(anyhow!("Unexpected subprotocol from server: {protocol:?}"));
        }

        let (tx, rx) = stream.split();
        let state = State::default();
        let rx_task = tokio::spawn(rx_task(rx, state.clone()));
        Ok(Self {
            tx: tokio::sync::Mutex::new(tx),
            state,
            rx_task,
        })
    }

    async fn send(&self, msg: Message) -> Result<()> {
        let mut tx = self.tx.lock().await;
        tx.send(msg).await?;
        Ok(())
    }

    async fn service_call(
        &self,
        service_name: &str,
        encoding: &str,
        payload: Bytes,
    ) -> Result<Bytes> {
        loop {
            let Some(service_id) = self.state.get_service_id(service_name) else {
                info!("Waiting for service {service_name} to be advertised");
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            };
            let call = self.state.service_call(service_id, encoding, payload);
            self.send(call.msg).await?;
            return call.rx.await?;
        }
    }

    async fn call_add(&self, a: u64, b: u64) -> Result<u64> {
        let req = serde_json::to_vec(&IntBinRequest { a, b })?;
        let resp = self.service_call("/IntBin/add", "json", req.into()).await?;
        let resp: IntBinResponse = serde_json::from_slice(&resp)?;
        Ok(resp.result)
    }

    async fn call_echo(&self, msg: String) -> Result<String> {
        let req = msg.into_bytes().into();
        let resp = self.service_call("/echo", "raw", req).await?;
        let resp = String::from_utf8(resp.to_vec())?;
        Ok(resp)
    }

    async fn call_sleep(&self) -> Result<()> {
        self.service_call("/sleep", "", Bytes::new()).await?;
        Ok(())
    }

    async fn call_set_flag(&self, data: bool) -> Result<SetBoolResponse> {
        let req = serde_json::to_vec(&SetBoolRequest { data })?;
        let resp = self.service_call("/flag_a", "json", req.into()).await?;
        let resp: SetBoolResponse = serde_json::from_slice(&resp)?;
        Ok(resp)
    }

    async fn close(&self) -> Result<()> {
        let mut tx = self.tx.lock().await;
        tx.close().await?;
        Ok(())
    }
}

pub async fn main(config: Config) -> Result<()> {
    let client = Arc::new(Client::connect(&config.host, config.port).await?);

    let sum = client.call_add(4, 38).await?;
    info!("got sum: {sum}");

    let echo = client.call_echo("echo! echo!".into()).await?;
    info!("got echo: {echo}");

    let result = client.call_set_flag(true).await?;
    info!("set flag: {result:?}");

    let result = client.call_set_flag(false).await?;
    info!("clear flag: {result:?}");

    // Launch a bunch of concurrent calls, expecting some of them to fail with "Too many requests".
    let mut sleepers = JoinSet::default();
    for id in 0..50 {
        let client = client.clone();
        sleepers.spawn(async move {
            if let Err(e) = client.call_sleep().await {
                error!("{id} failed to sleep: {e}");
            } else {
                info!("{id} is awake");
            }
        });
    }

    let _ = sleepers.join_all().await;

    client.close().await
}
