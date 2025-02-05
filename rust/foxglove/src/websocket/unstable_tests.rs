use std::{collections::HashSet, sync::Arc};

use bytes::{Buf, BufMut, BytesMut};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::tungstenite::Message;

use super::tests::connect_client;
use super::{
    create_server, protocol, Capability, ClientChannelId, Parameter, ParameterType, ParameterValue,
    ServerListener, ServerOptions,
};

struct ClientMessageData {
    channel_id: ClientChannelId,
    payload: Vec<u8>,
}

struct MPSCServerListener(UnboundedSender<ClientMessageData>);

impl MPSCServerListener {
    fn create() -> (
        Arc<dyn ServerListener>,
        UnboundedReceiver<ClientMessageData>,
    ) {
        let (tx, rx) = unbounded_channel();
        (Arc::new(Self(tx)), rx)
    }
}

impl ServerListener for MPSCServerListener {
    fn on_message_data(&self, channel_id: ClientChannelId, message: &[u8]) {
        self.0
            .send(ClientMessageData {
                channel_id,
                payload: message.to_vec(),
            })
            .expect("MPSC queue closed");
    }
}

#[tokio::test]
async fn test_client_advertising() {
    let (listener, mut chan_rx) = MPSCServerListener::create();

    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([Capability::ClientPublish])),
        supported_encodings: Some(HashSet::from(["json".to_string()])),
        listener: Some(listener),
        session_id: None,
        name: None,
        message_backlog_size: None,
    });

    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    let channel_id = 1;
    let msg_bytes = {
        let mut bytes = BytesMut::new();
        bytes.put_u8(0x01); // message data opcode
        bytes.put_u32_le(channel_id);
        bytes.put_slice(json!({ "a": 1 }).to_string().as_bytes());
        bytes
    };

    // Send before advertising: message is dropped
    ws_client
        .send(Message::binary(msg_bytes.clone()))
        .await
        .expect("Failed to send binary message");
    // No message sent to listener
    assert!(chan_rx.try_recv().is_err());

    let advertise = json!({
        "op": "advertise",
        "channels": [
            {
                "id": channel_id,
                "topic": "/test",
                "encoding": "json",
                "schemaName": "test",
            }
        ]
    });

    ws_client
        .send(Message::text(advertise.to_string()))
        .await
        .expect("Failed to send advertisement");

    // Send duplicate advertisement: no effect
    ws_client
        .send(Message::text(advertise.to_string()))
        .await
        .expect("Failed to send duplicate advertisement");

    // Send message after advertising
    ws_client
        .send(Message::binary(msg_bytes))
        .await
        .expect("Failed to send binary message");

    // Does not error on a binary message with no opcode
    ws_client
        .send(Message::binary(vec![]))
        .await
        .expect("Failed to send empty binary message");

    let unadvertise = json!({
        "op": "unadvertise",
        "channelIds": [channel_id]
    });

    tracing::info!("unadvertise: {}", unadvertise.to_string());
    ws_client
        .send(Message::text(unadvertise.to_string()))
        .await
        .expect("Failed to send unadvertise");

    // Server should have received one message
    let received = chan_rx.recv().await.expect("No message received");
    assert_eq!(received.channel_id, ClientChannelId::new(1));
    assert_eq!(received.payload, b"{\"a\":1}");

    ws_client.close(None).await.unwrap();
    server.stop().await;
}

#[tokio::test]
async fn test_broadcast_time() {
    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([Capability::Time])),
        ..Default::default()
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;
    _ = ws_client.next().await.expect("serverInfo");

    server.broadcast_time(42).await;
    let msg = ws_client
        .next()
        .await
        .expect("no message received")
        .expect("failed to parse message");
    let Message::Binary(mut buf) = msg else {
        panic!("unexpected message type");
    };
    assert_eq!(buf.get_u8(), protocol::server::BinaryOpcode::TimeData as u8);
    assert_eq!(buf.get_u64_le(), 42);
}

#[tokio::test]
async fn test_parameter_values() {
    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([Capability::Parameters])),
        listener: None,
        session_id: None,
        name: None,
        message_backlog_size: None,
        supported_encodings: None,
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    _ = ws_client.next().await.expect("No serverInfo sent");

    let parameter = Parameter {
        name: "some-float-value".to_string(),
        value: Some(ParameterValue::Number(1.23)),
        r#type: Some(ParameterType::Float64),
    };
    server.publish_parameter_values(vec![parameter], None).await;

    // Allow the server to process the parameter values
    std::thread::sleep(std::time::Duration::from_millis(100));

    let msg = ws_client.next().await.expect("No message received");
    let msg = msg.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let msg: Value = serde_json::from_str(&text).expect("Failed to parse message");
    assert_eq!(msg["op"], "parameterValues");
    assert_eq!(msg["parameters"][0]["name"], "some-float-value");
    assert_eq!(msg["parameters"][0]["value"], 1.23);

    server.stop().await;
}
