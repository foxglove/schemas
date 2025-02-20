use assert_matches::assert_matches;
use bytes::{BufMut, BytesMut};
use futures_util::{FutureExt, SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::{self, http::HeaderValue, Message};
use tracing_test::traced_test;
use tungstenite::client::IntoClientRequest;

use super::{create_server, send_lossy, SendLossyResult, ServerOptions, SUBPROTOCOL};
use crate::testutil::RecordingServerListener;
use crate::websocket::service::{CallId, Service, ServiceId, ServiceSchema};
use crate::websocket::{
    Capability, ClientChannelId, Parameter, ParameterType, ParameterValue, Status, StatusLevel,
};
use crate::{
    collection, Channel, ChannelBuilder, FoxgloveError, LogContext, LogSink, Metadata, Schema,
};

fn make_message(id: usize) -> Message {
    Message::Text(format!("{id}").into())
}

fn parse_message(msg: Message) -> usize {
    match msg {
        Message::Text(text) => text.parse().expect("id"),
        _ => unreachable!(),
    }
}

#[derive(Debug, Deserialize)]
struct ParameterValues {
    id: Option<String>,
    parameters: Vec<Parameter>,
}

#[traced_test]
#[test]
fn test_send_lossy() {
    const BACKLOG: usize = 4;
    const TOTAL: usize = 10;

    let addr = SocketAddr::new("127.0.0.1".parse().unwrap(), 1234);

    let (tx, rx) = flume::bounded(BACKLOG);
    for i in 0..BACKLOG {
        assert_matches!(
            send_lossy(&addr, &tx, &rx, make_message(i), 0),
            SendLossyResult::Sent
        );
    }

    // The queue is full now. We'll only succeed with retries.
    for i in BACKLOG..TOTAL {
        assert_matches!(
            send_lossy(&addr, &tx, &rx, make_message(TOTAL + i), 0),
            SendLossyResult::ExhaustedRetries
        );
        assert_matches!(
            send_lossy(&addr, &tx, &rx, make_message(i), 1),
            SendLossyResult::SentLossy(1)
        );
    }

    // Receive everything, expect that the first (TOTAL - BACKLOG) messages were dropped.
    let mut received: Vec<usize> = vec![];
    while let Ok(msg) = rx.try_recv() {
        received.push(parse_message(msg));
    }
    assert_eq!(received, ((TOTAL - BACKLOG)..TOTAL).collect::<Vec<_>>());
}

fn new_channel(topic: &str, ctx: &LogContext) -> Arc<Channel> {
    ChannelBuilder::new(topic)
        .message_encoding("message_encoding")
        .schema(Schema::new(
            "schema_name",
            "schema_encoding",
            b"schema_data",
        ))
        .metadata(collection! {"key".to_string() => "value".to_string()})
        .with_context(ctx)
        .build()
        .expect("Failed to create channel")
}

#[traced_test]
#[tokio::test]
async fn test_client_connect() {
    let server = create_server(ServerOptions {
        session_id: Some("mock_sess_id".to_string()),
        name: Some("mock_server".to_string()),
        ..Default::default()
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut client_stream = connect_client(addr).await;

    let result = client_stream.next().await.expect("No message received");
    let msg = result.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let server_info: Value = serde_json::from_str(&text).expect("Failed to parse server info");

    assert_eq!(server_info["name"], "mock_server");
    assert_eq!(server_info["sessionId"], "mock_sess_id");

    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_handshake_with_unknown_subprotocol_fails_on_client() {
    let server = create_server(ServerOptions::default());
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut request = format!("ws://{addr}/")
        .into_client_request()
        .expect("Failed to build request");

    request.headers_mut().insert(
        "sec-websocket-protocol",
        HeaderValue::from_static("unknown"),
    );

    let result = tokio_tungstenite::connect_async(request).await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "WebSocket protocol error: SubProtocol error: Server sent no subprotocol"
    );
}

#[traced_test]
#[tokio::test]
async fn test_handshake_with_multiple_subprotocols() {
    let server = create_server(ServerOptions::default());
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let request = format!("ws://{addr}/")
        .into_client_request()
        .expect("Failed to build request");

    let mut req1 = request.clone();
    let header = format!("{}, foxglove.sdk.v2", SUBPROTOCOL);
    req1.headers_mut().insert(
        "sec-websocket-protocol",
        HeaderValue::from_str(&header).unwrap(),
    );

    let (_, response) = tokio_tungstenite::connect_async(req1)
        .await
        .expect("Failed to connect");

    assert_eq!(
        response.headers().get("sec-websocket-protocol"),
        Some(&HeaderValue::from_static(SUBPROTOCOL))
    );

    // In req2, the client's preferred (initial) subprotocol is not valid
    let mut req2 = request.clone();
    let header = format!("unknown, {}, another", SUBPROTOCOL);
    req2.headers_mut().insert(
        "sec-websocket-protocol",
        HeaderValue::from_str(&header).unwrap(),
    );

    let (_, response) = tokio_tungstenite::connect_async(req2)
        .await
        .expect("Failed to connect");

    assert_eq!(
        response.headers().get("sec-websocket-protocol"),
        Some(&HeaderValue::from_static(SUBPROTOCOL))
    );

    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_advertise_to_client() {
    let recording_listener = Arc::new(RecordingServerListener::new());

    let server = create_server(ServerOptions {
        listener: Some(recording_listener.clone()),
        ..Default::default()
    });

    let ctx = LogContext::new();
    ctx.add_sink(server.clone());

    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let client_stream = connect_client(addr).await;
    let (mut client_sender, mut client_receiver) = client_stream.split();

    let msg = client_receiver.next().await.expect("No serverInfo sent");
    msg.expect("Invalid serverInfo");

    let ch = new_channel("/foo", &ctx);
    let metadata = Metadata::default();

    server.log(&ch, b"foo bar", &metadata).unwrap();

    let subscription_id = 1;
    let result = client_receiver.next().await.expect("No advertisement sent");
    let advertisement = result.expect("Failed to parse advertisement");
    let text = advertisement.into_text().expect("Invalid advertisement");
    let msg: Value = serde_json::from_str(&text).expect("Failed to advertisement");
    assert_eq!(msg["op"], "advertise");
    assert_eq!(
        msg["channels"][0]["id"].as_u64().unwrap(),
        u64::from(ch.id())
    );

    let subscribe = json!({
        "op": "subscribe",
        "subscriptions": [
            {
                "id": subscription_id,
                "channelId": ch.id(),
            }
        ]
    });
    client_sender
        .send(Message::text(subscribe.to_string()))
        .await
        .expect("Failed to send");
    // Send a duplicate subscribe message (ignored)
    client_sender
        .send(Message::text(subscribe.to_string()))
        .await
        .expect("Failed to send");

    // Allow the server to process the subscription
    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    server.log(&ch, b"{\"a\":1}", &metadata).unwrap();

    let result = client_receiver.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let data = msg.into_data();
    let data_str = std::str::from_utf8(&data).unwrap();
    assert!(data_str.contains("Client is already subscribed to channel"));

    let msg = client_receiver
        .next()
        .await
        .unwrap()
        .expect("Failed to parse message");
    let data = msg.into_data();

    assert_eq!(data[0], 0x01); // message data opcode
    assert_eq!(
        u32::from_le_bytes(data[1..=4].try_into().unwrap()),
        subscription_id
    );

    let subscriptions = recording_listener.take_subscribe();
    assert_eq!(subscriptions.len(), 1);
    assert_eq!(subscriptions[0].1.id, ch.id);
    assert_eq!(subscriptions[0].1.topic, ch.topic);

    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_log_only_to_subscribers() {
    let recording_listener = Arc::new(RecordingServerListener::new());

    let server = create_server(ServerOptions {
        listener: Some(recording_listener.clone()),
        ..Default::default()
    });

    let ctx = LogContext::new();

    ctx.add_sink(server.clone());

    let ch1 = new_channel("/foo", &ctx);
    let ch2 = new_channel("/bar", &ctx);
    let ch3 = new_channel("/baz", &ctx);

    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut client1 = connect_client(addr.clone()).await;
    let mut client2 = connect_client(addr.clone()).await;
    let mut client3 = connect_client(addr).await;

    // client1 subscribes to ch1; client2 subscribes to ch2; client3 unsubscribes from all
    // Read the server info message from each
    let _ = client1.next().await.expect("No serverInfo sent").unwrap();
    let _ = client2.next().await.expect("No serverInfo sent").unwrap();
    let _ = client3.next().await.expect("No serverInfo sent").unwrap();

    // Read the channel advertisement from each client for all 3 channels
    for _ in 0..3 {
        let _ = client1
            .next()
            .await
            .expect("No advertisement sent")
            .unwrap();
        let _ = client2
            .next()
            .await
            .expect("No advertisement sent")
            .unwrap();
        let _ = client3
            .next()
            .await
            .expect("No advertisement sent")
            .unwrap();
    }

    let subscribe1 = json!({
        "op": "subscribe",
        "subscriptions": [
            {
                "id": 1,
                "channelId": ch1.id(),
            }
        ]
    });
    client1
        .send(Message::text(subscribe1.to_string()))
        .await
        .expect("Failed to send");

    let subscribe2 = json!({
        "op": "subscribe",
        "subscriptions": [
            {
                "id": 2,
                "channelId": ch2.id(),
            }
        ]
    });
    client2
        .send(Message::text(subscribe2.to_string()))
        .await
        .expect("Failed to send");

    let unsubscribe_both = json!({
        "op": "unsubscribe",
        "subscriptionIds": [1, 2],
    });
    client3
        .send(Message::text(subscribe1.to_string()))
        .await
        .expect("Failed to send");
    client3
        .send(Message::text(subscribe2.to_string()))
        .await
        .expect("Failed to send");
    client3
        .send(Message::text(unsubscribe_both.to_string()))
        .await
        .expect("Failed to send");

    // Allow the server to process the subscription
    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let subscriptions = recording_listener.take_subscribe();
    assert_eq!(subscriptions.len(), 4);
    assert_eq!(subscriptions[0].1.id, ch1.id);
    assert_eq!(subscriptions[1].1.id, ch2.id);
    assert_eq!(subscriptions[2].1.id, ch1.id);
    assert_eq!(subscriptions[3].1.id, ch2.id);
    assert_eq!(subscriptions[0].1.topic, ch1.topic);
    assert_eq!(subscriptions[1].1.topic, ch2.topic);
    assert_eq!(subscriptions[2].1.topic, ch1.topic);
    assert_eq!(subscriptions[3].1.topic, ch2.topic);

    let unsubscriptions = recording_listener.take_unsubscribe();
    assert_eq!(unsubscriptions.len(), 2);
    assert_eq!(unsubscriptions[0].1.id, ch1.id);
    assert_eq!(unsubscriptions[1].1.id, ch2.id);
    assert_eq!(unsubscriptions[0].1.topic, ch1.topic);
    assert_eq!(unsubscriptions[1].1.topic, ch2.topic);

    let metadata = Metadata {
        log_time: 123456,
        ..Metadata::default()
    };
    server.log(&ch1, b"channel1", &metadata).unwrap();
    server.log(&ch2, b"channel2", &metadata).unwrap();
    server.log(&ch3, b"channel3", &metadata).unwrap();

    // Receive the message for client1 and client2
    let result = client1.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let data = msg.into_data();
    assert_eq!(data[0], 0x01); // message data opcode
    assert_eq!(u32::from_le_bytes(data[1..=4].try_into().unwrap()), 1);
    assert_eq!(u64::from_le_bytes(data[5..=12].try_into().unwrap()), 123456);
    assert_eq!(&data[13..], b"channel1");

    let result = client2.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let data = msg.into_data();
    assert_eq!(data[0], 0x01); // message data opcode
    assert_eq!(u32::from_le_bytes(data[1..=4].try_into().unwrap()), 2);
    assert_eq!(u64::from_le_bytes(data[5..=12].try_into().unwrap()), 123456);
    assert_eq!(&data[13..], b"channel2");

    // Client 3 should not receive any messages since it unsubscribed from all channels
    let rs = client3.next().now_or_never();
    assert!(rs.is_none());

    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_error_when_client_publish_unsupported() {
    // Server does not support clientPublish capability by default
    let server = create_server(ServerOptions::default());
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;
    ws_client.next().await.expect("No serverInfo sent").ok();

    let advertise = json!({
        "op": "advertise",
        "channels": [
            {
                "id": 1,
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

    // Server should respond with an error status
    let result = ws_client.next().await.expect("No message received");
    let msg = result.expect("Failed to parse message");
    let msg = msg.into_text().expect("Failed to parse message text");
    let status: Value = serde_json::from_str(&msg).expect("Failed to parse status");
    assert_eq!(status["op"], "status");
    assert_eq!(status["level"], 2);
    assert_eq!(
        status["message"],
        "Server does not support clientPublish capability"
    );

    ws_client.close(None).await.unwrap();
    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_error_status_message() {
    let server = create_server(ServerOptions::default());
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    _ = ws_client.next().await.expect("No serverInfo sent");

    {
        ws_client
            .send(Message::text("nonsense".to_string()))
            .await
            .expect("Failed to send message");

        let result = ws_client.next().await.unwrap();
        let msg = result.expect("Failed to parse message");
        let text = msg.into_text().expect("Failed to get message text");
        let status: Value = serde_json::from_str(&text).expect("Failed to parse status");
        assert_eq!(status["level"], 2);
        assert_eq!(
            status["message"],
            "Invalid message: expected ident at line 1 column 2"
        );
    }

    {
        let msg = json!({
            "op": "subscribe",
            "subscriptions": [{ "id": 1, "channelId": 555, }]
        });
        ws_client
            .send(Message::text(msg.to_string()))
            .await
            .expect("Failed to send message");

        let result = ws_client.next().await.unwrap();
        let msg = result.expect("Failed to parse message");
        let text = msg.into_text().expect("Failed to get message text");
        let status: Value = serde_json::from_str(&text).expect("Failed to parse status");
        assert_eq!(status["level"], 2);
        assert_eq!(status["message"], "Unknown channel ID: 555");
    }

    {
        ws_client
            .send(Message::binary(vec![0xff]))
            .await
            .expect("Failed to send message");

        let result = ws_client.next().await.unwrap();
        let msg = result.expect("Failed to parse message");
        let text = msg.into_text().expect("Failed to get message text");
        let status: Value = serde_json::from_str(&text).expect("Failed to parse status");
        assert_eq!(status["level"], 2);
        assert_eq!(
            status["message"],
            "Invalid message: Unknown binary opcode 255"
        );
    }

    server.stop().await;
}

#[tokio::test]
async fn test_service_registration_not_supported() {
    // Can't register services if we don't declare support.
    let server = create_server(ServerOptions::default());
    let svc = Service::builder("/s", ServiceSchema::new("")).sync_handler_fn(|_, _| Err(""));
    assert_matches!(
        server.add_services(vec![svc]),
        Err(FoxgloveError::ServicesNotSupported)
    );
}

#[tokio::test]
async fn test_service_registration_missing_request_encoding() {
    // Can't register a service with no encoding unless we declare global encodings.
    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([Capability::Services])),
        ..Default::default()
    });
    let svc = Service::builder("/s", ServiceSchema::new("")).sync_handler_fn(|_, _| Err(""));
    assert_matches!(
        server.add_services(vec![svc]),
        Err(FoxgloveError::MissingRequestEncoding(_))
    );
}

#[tokio::test]
async fn test_service_registration_duplicate_name() {
    // Can't register a service with no encoding unless we declare global encodings.
    let sa1 = Service::builder("/a", ServiceSchema::new("")).sync_handler_fn(|_, _| Err(""));
    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([Capability::Services])),
        services: HashMap::from([(sa1.name().to_string(), sa1)]),
        supported_encodings: Some(HashSet::from(["ros1msg".into()])),
        ..Default::default()
    });

    let sa2 = Service::builder("/a", ServiceSchema::new("")).sync_handler_fn(|_, _| Err(""));
    assert_matches!(
        server.add_services(vec![sa2]),
        Err(FoxgloveError::DuplicateService(_))
    );

    let sb1 = Service::builder("/b", ServiceSchema::new("")).sync_handler_fn(|_, _| Err(""));
    let sb2 = Service::builder("/b", ServiceSchema::new("")).sync_handler_fn(|_, _| Err(""));
    assert_matches!(
        server.add_services(vec![sb1, sb2]),
        Err(FoxgloveError::DuplicateService(_))
    );
}

#[traced_test]
#[tokio::test]
async fn test_publish_status_message() {
    let server = create_server(ServerOptions::default());

    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    _ = ws_client.next().await.expect("No serverInfo sent");

    server
        .publish_status(Status::new(StatusLevel::Info, "Hello, world!".to_string()).with_id("123"));

    let msg = ws_client
        .next()
        .await
        .expect("No message received")
        .expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    assert_eq!(
        text.as_str(),
        r#"{"op":"status","level":0,"message":"Hello, world!","id":"123"}"#
    );

    server.publish_status(
        Status::new(StatusLevel::Error, "Reactor core overload!".to_string()).with_id("abc"),
    );

    let msg = ws_client
        .next()
        .await
        .expect("No message received")
        .expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    assert_eq!(
        text.as_str(),
        r#"{"op":"status","level":2,"message":"Reactor core overload!","id":"abc"}"#
    );
}

#[traced_test]
#[tokio::test]
async fn test_remove_status() {
    let server = create_server(ServerOptions::default());
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client1 = connect_client(addr.clone()).await;
    let mut ws_client2 = connect_client(addr).await;

    _ = ws_client1.next().await.expect("No serverInfo sent");
    _ = ws_client2.next().await.expect("No serverInfo sent");

    // These don't have to exist, and aren't checked
    server.remove_status(vec!["123".to_string(), "abc".to_string()]);

    let msg = ws_client1
        .next()
        .await
        .expect("No message received")
        .expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    assert_eq!(
        text.as_str(),
        r#"{"op":"removeStatus","statusIds":["123","abc"]}"#
    );

    let msg = ws_client2
        .next()
        .await
        .expect("No message received")
        .expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    assert_eq!(
        text.as_str(),
        r#"{"op":"removeStatus","statusIds":["123","abc"]}"#
    );
}

#[traced_test]
#[tokio::test]
async fn test_client_advertising() {
    let recording_listener = Arc::new(RecordingServerListener::new());

    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([Capability::ClientPublish])),
        supported_encodings: Some(HashSet::from(["json".to_string()])),
        listener: Some(recording_listener.clone()),
        ..Default::default()
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
    assert!(recording_listener.take_message_data().is_empty());

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

    // Should be ignored
    ws_client
        .send(Message::text(unadvertise.to_string()))
        .await
        .expect("Failed to send unadvertise");

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    // Server should have received one message
    let mut received = recording_listener.take_message_data();
    let message_data = received.pop().expect("No message received");
    assert_eq!(message_data.channel.id, ClientChannelId::new(1));
    assert_eq!(message_data.data, b"{\"a\":1}");

    // Server should have ignored the duplicate advertisement
    let advertisements = recording_listener.take_client_advertise();
    assert_eq!(advertisements.len(), 1);
    assert_eq!(advertisements[0].1.id, ClientChannelId::new(channel_id));

    // Server should have received one unadvertise (and ignored the duplicate)
    let unadvertises = recording_listener.take_client_unadvertise();
    assert_eq!(unadvertises.len(), 1);
    assert_eq!(unadvertises[0].1.id, ClientChannelId::new(channel_id));

    ws_client.close(None).await.unwrap();
    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_parameter_values() {
    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([
            Capability::Parameters,
            Capability::ParametersSubscribe,
        ])),
        ..Default::default()
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    // Send the Subscribe Parameter Update message for "some-float-value"
    // Otherwise we won't get the update after we publish it.
    ws_client
        .send(Message::text(
            r#"{"op":"subscribeParameterUpdates","parameterNames":["some-float-value"]}"#,
        ))
        .await
        .expect("Failed to send subscribe parameter updates");

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let parameter = Parameter {
        name: "some-float-value".to_string(),
        value: Some(ParameterValue::Number(1.23)),
        r#type: Some(ParameterType::Float64),
    };
    server.publish_parameter_values(vec![parameter]);

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    _ = ws_client.next().await.expect("No serverInfo sent");

    let msg = ws_client.next().await.expect("No message received");
    let msg = msg.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let msg: Value = serde_json::from_str(&text).expect("Failed to parse message");
    assert_eq!(msg["op"], "parameterValues");
    assert_eq!(msg["parameters"].as_array().unwrap().len(), 1);
    assert_eq!(msg["parameters"][0]["name"], "some-float-value");
    assert_eq!(msg["parameters"][0]["value"], 1.23);

    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_parameter_unsubscribe_no_updates() {
    let recording_listener = Arc::new(RecordingServerListener::new());

    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([
            Capability::Parameters,
            Capability::ParametersSubscribe,
        ])),
        listener: Some(recording_listener.clone()),
        ..Default::default()
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    // Send the Subscribe Parameter Update message for "some-float-value"
    ws_client
        .send(Message::text(
            r#"{"op":"subscribeParameterUpdates","parameterNames":["some-float-value"]}"#,
        ))
        .await
        .expect("Failed to send subscribe parameter updates");

    // Send the Unsubscribe Parameter Update message for "some-float-value"
    ws_client
        .send(Message::text(
            r#"{"op":"unsubscribeParameterUpdates","parameterNames":["some-float-value","baz"]}"#,
        ))
        .await
        .expect("Failed to send unsubscribe parameter updates");

    _ = ws_client.next().await.expect("No serverInfo sent");

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let parameter_names = recording_listener
        .take_parameters_subscribe()
        .pop()
        .unwrap();
    assert_eq!(parameter_names, vec!["some-float-value"]);

    let parameter_names = recording_listener
        .take_parameters_unsubscribe()
        .pop()
        .unwrap();
    assert_eq!(parameter_names, vec!["some-float-value"]);

    let parameter = Parameter {
        name: "some-float-value".to_string(),
        value: Some(ParameterValue::Number(1.23)),
        r#type: Some(ParameterType::Float64),
    };
    server.publish_parameter_values(vec![parameter]);

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    server.stop().await;

    // No parameter message was sent with the updated param before the Close message
    assert!(matches!(
        ws_client.next().await,
        Some(Ok(Message::Close(_)))
    ));
}

#[traced_test]
#[tokio::test]
async fn test_set_parameters() {
    let recording_listener = Arc::new(RecordingServerListener::new());

    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([
            Capability::Parameters,
            Capability::ParametersSubscribe,
        ])),
        listener: Some(recording_listener.clone()),
        ..Default::default()
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    // Subscribe to "foo" and "bar"
    ws_client
        .send(Message::text(
            r#"{"op":"subscribeParameterUpdates","parameterNames":["foo", "bar"]}"#,
        ))
        .await
        .expect("Failed to send subscribe parameter updates");

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    ws_client
        .send(Message::text(
            r#"{"op":"setParameters", "parameters":[{"name":"foo","value":1,"type":"float64"},{"name":"bar","value":"aGVsbG8="},{"name":"baz","value":true}], "id":"123"}"#,
        ))
        .await
        .expect("Failed to send set parameters");

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let set_parameters = recording_listener.take_parameters_set().pop().unwrap();
    assert_eq!(set_parameters.parameters.len(), 3);
    assert_eq!(set_parameters.parameters[0].name, "foo");
    assert_eq!(
        set_parameters.parameters[0].value,
        Some(ParameterValue::Number(1.0))
    );
    assert_eq!(
        set_parameters.parameters[0].r#type,
        Some(ParameterType::Float64)
    );
    assert_eq!(set_parameters.parameters[1].name, "bar");
    assert_eq!(
        set_parameters.parameters[1].value,
        Some(ParameterValue::String(Vec::from("hello".as_bytes())))
    );
    assert_eq!(set_parameters.parameters[1].r#type, None);
    assert_eq!(set_parameters.parameters[2].name, "baz");
    assert_eq!(
        set_parameters.parameters[2].value,
        Some(ParameterValue::Bool(true))
    );
    assert_eq!(set_parameters.parameters[2].r#type, None);
    assert_eq!(set_parameters.request_id, Some("123".to_string()));

    _ = ws_client.next().await.expect("No serverInfo sent");

    // setParameters returns the result of on_set_parameters, which for recording listener, just returns them back
    let msg = ws_client.next().await.expect("No message received");
    let msg = msg.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let msg: ParameterValues = serde_json::from_str(&text).expect("Failed to parse message");
    let params = msg.parameters;
    assert_eq!(params.len(), 3);
    assert_eq!(params[0].name, "foo");
    assert_eq!(params[0].value, Some(ParameterValue::Number(1.0)));
    assert_eq!(params[0].r#type, Some(ParameterType::Float64));
    assert_eq!(params[1].name, "bar");
    assert_eq!(
        params[1].value,
        Some(ParameterValue::String(Vec::from("hello".as_bytes())))
    );
    assert_eq!(params[1].r#type, None);
    assert_eq!(params[2].name, "baz");
    assert_eq!(params[2].value, Some(ParameterValue::Bool(true)));
    assert_eq!(params[2].r#type, None);

    // it will also publish the updated paramters returned from on_set_parameters
    // which will send just the paramters we're subscribed to.
    let msg = ws_client.next().await.expect("No message received");
    let msg = msg.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let msg: ParameterValues = serde_json::from_str(&text).expect("Failed to parse message");
    let params = msg.parameters;
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name, "foo");
    assert_eq!(params[0].value, Some(ParameterValue::Number(1.0)));
    assert_eq!(params[0].r#type, Some(ParameterType::Float64));
    assert_eq!(params[1].name, "bar");
    assert_eq!(
        params[1].value,
        Some(ParameterValue::String(Vec::from("hello".as_bytes())))
    );
    assert_eq!(params[1].r#type, None);

    server.stop().await;
}

#[traced_test]
#[tokio::test]
async fn test_get_parameters() {
    let recording_listener = Arc::new(RecordingServerListener::new());
    recording_listener.set_parameters_get_result(vec![Parameter {
        name: "foo".to_string(),
        value: Some(ParameterValue::Number(1.0)),
        r#type: Some(ParameterType::Float64),
    }]);

    let server = create_server(ServerOptions {
        capabilities: Some(HashSet::from([Capability::Parameters])),
        listener: Some(recording_listener.clone()),
        ..Default::default()
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    ws_client
        .send(Message::text(
            r#"{"op":"getParameters", "parameterNames":["foo", "bar", "baz"], "id":"123"}"#,
        ))
        .await
        .expect("Failed to send get parameters");

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let get_parameters = recording_listener.take_parameters_get().pop().unwrap();
    assert_eq!(get_parameters.param_names, vec!["foo", "bar", "baz"]);
    assert_eq!(get_parameters.request_id, Some("123".to_string()));

    _ = ws_client.next().await.expect("No serverInfo sent");

    let msg = ws_client.next().await.expect("No message received");
    let msg = msg.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let msg: ParameterValues = serde_json::from_str(&text).expect("Failed to parse message");
    let params = msg.parameters;
    assert_eq!(msg.id, Some("123".to_string()));
    assert_eq!(params.len(), 1);
    assert_eq!(params[0].name, "foo");
    assert_eq!(params[0].value, Some(ParameterValue::Number(1.0)));
    assert_eq!(params[0].r#type, Some(ParameterType::Float64));

    server.stop().await;
}

#[tokio::test]
async fn test_services() {
    let ok_svc = Service::builder("/ok", ServiceSchema::new("plain"))
        .with_id(ServiceId::new(1))
        .handler_fn(|_, req, resp| {
            assert_eq!(req.service_id(), ServiceId::new(1));
            assert_eq!(req.service_name(), "/ok");
            assert_eq!(req.call_id(), CallId::new(99));
            let payload = req.into_payload();
            let mut response = BytesMut::with_capacity(payload.len());
            response.put(payload);
            response.reverse();
            // Respond async, for kicks.
            tokio::spawn(async move { resp.respond(Ok(response.freeze())) });
        });

    let server = create_server(ServerOptions {
        services: [ok_svc]
            .into_iter()
            .map(|s| (s.name().to_string(), s))
            .collect(),
        supported_encodings: Some(HashSet::from(["raw".to_string()])),
        ..Default::default()
    });

    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut client1 = connect_client(addr.clone()).await;
    let _ = client1.next().await.expect("No serverInfo sent").unwrap();
    let msg = client1
        .next()
        .await
        .expect("No service advertisement sent")
        .unwrap();
    assert_eq!(
        msg.into_text().expect("Expected utf8").as_str(),
        json!({
            "op": "advertiseServices",
            "services": [
                {
                    "id": 1,
                    "name": "/ok",
                    "type": "plain",
                    "requestSchema": "",
                    "responseSchema": "",
                }
            ]
        })
        .to_string()
    );

    // Send a request.
    let mut buf = BytesMut::new();
    buf.put_u8(2); // opcode
    buf.put_u32_le(1); // service id
    buf.put_u32_le(99); // call id
    buf.put_u32_le(3); // encoding length
    buf.put(b"raw".as_slice());
    buf.put(b"payload".as_slice());
    let ok_req = buf.freeze();
    client1
        .send(Message::binary(ok_req.clone()))
        .await
        .expect("Failed to send");

    // Validate the response.
    let msg = client1
        .next()
        .await
        .expect("No service call response")
        .expect("Failed to parse response");
    let mut buf = BytesMut::new();
    buf.put_u8(3); // opcode
    buf.put_u32_le(1); // service id
    buf.put_u32_le(99); // call id
    buf.put_u32_le(3); // encoding length
    buf.put(b"raw".as_slice());
    buf.put(b"daolyap".as_slice());
    assert_eq!(msg.into_data(), buf);

    // Register a new service.
    let err_svc = Service::builder("/err", ServiceSchema::new("plain"))
        .with_id(ServiceId::new(2))
        .sync_handler_fn(|_, _| Err("oh noes"));
    server
        .add_services(vec![err_svc])
        .expect("Failed to add service");

    let msg = client1
        .next()
        .await
        .expect("No service advertisement sent")
        .unwrap();
    assert_eq!(
        msg.into_text().expect("Expected utf8").as_str(),
        json!({
            "op": "advertiseServices",
            "services": [
                {
                    "id": 2,
                    "name": "/err",
                    "type": "plain",
                    "requestSchema": "",
                    "responseSchema": "",
                }
            ]
        })
        .to_string()
    );

    // Send a request to the error service.
    let mut buf = BytesMut::new();
    buf.put_u8(2); // opcode
    buf.put_u32_le(2); // service id
    buf.put_u32_le(11); // call id
    buf.put_u32_le(3); // encoding length
    buf.put(b"raw".as_slice());
    buf.put(b"payload".as_slice());
    client1
        .send(Message::binary(buf.freeze()))
        .await
        .expect("Failed to send");

    // Validate the error response.
    let msg = client1
        .next()
        .await
        .expect("No service call response")
        .expect("Failed to parse response");
    assert_eq!(
        msg.into_text().expect("Expected utf8").as_str(),
        json!({
            "op": "serviceCallFailure",
            "serviceId": 2,
            "callId": 11,
            "message": "oh noes",
        })
        .to_string()
    );

    // New client sees both services immediately.
    let mut client2 = connect_client(addr.clone()).await;
    let _ = client2.next().await.expect("No serverInfo sent").unwrap();
    let msg = client2
        .next()
        .await
        .expect("No service advertisement sent")
        .unwrap();
    let value: serde_json::Value =
        serde_json::from_str(msg.into_text().expect("utf8").as_str()).expect("json");
    let adv_services = value
        .get("services")
        .and_then(|s| s.as_array())
        .expect("services key");
    assert_eq!(adv_services.len(), 2);
    drop(client2);

    // Unregister services.
    server.remove_services(&[ServiceId::new(1)]);
    let msg = client1
        .next()
        .await
        .expect("No service unadvertisement sent")
        .unwrap();
    assert_eq!(
        msg.into_text().expect("Expected utf8").as_str(),
        json!({
            "op": "unadvertiseServices",
            "serviceIds": [1]
        })
        .to_string()
    );

    // Try to call the now-unregistered service.
    client1
        .send(Message::binary(ok_req.clone()))
        .await
        .expect("Failed to send");

    // Validate the error response.
    let msg = client1
        .next()
        .await
        .expect("No service call response")
        .expect("Failed to parse response");
    assert_eq!(
        msg.into_text().expect("Expected utf8").as_str(),
        json!({
            "op": "serviceCallFailure",
            "serviceId": 1,
            "callId": 99,
            "message": "Unknown service",
        })
        .to_string()
    );
}

#[tokio::test]
async fn test_fetch_asset() {
    let recording_listener = Arc::new(RecordingServerListener::new());

    let server = create_server(ServerOptions {
        listener: Some(recording_listener.clone()),
        capabilities: Some(HashSet::from([Capability::Assets])),
        ..Default::default()
    });
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;
    let _ = ws_client.next().await.expect("No serverInfo sent");

    let fetch_asset = json!({
        "op": "fetchAsset",
        "uri": "https://example.com/asset.png",
        "requestId": 1,
    });
    ws_client
        .send(Message::text(fetch_asset.to_string()))
        .await
        .expect("Failed to send fetch asset");
    let fetch_asset_err = json!({
        "op": "fetchAsset",
        "uri": "https://example.com/error",
        "requestId": 2,
    });
    ws_client
        .send(Message::text(fetch_asset_err.to_string()))
        .await
        .expect("Failed to send fetch asset");

    // FG-10395 replace this with something more precise
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let result = ws_client.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let data = msg.into_data();
    assert_eq!(data[0], 0x04); // fetch asset opcode
    assert_eq!(u32::from_le_bytes(data[1..=4].try_into().unwrap()), 1);
    assert_eq!(data[5], 0); // 0 for success
    assert_eq!(&data[6..], b"test data");

    let result = ws_client.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let data = msg.into_data();
    assert_eq!(data[0], 0x04); // fetch asset opcode
    assert_eq!(u32::from_le_bytes(data[1..=4].try_into().unwrap()), 2);
    assert_eq!(data[5], 1); // 1 for error
    assert_eq!(&data[6..], b"test error");

    let fetch_asset = recording_listener.take_fetch_asset();
    assert_eq!(fetch_asset.len(), 2);
    assert_eq!(fetch_asset[0], "https://example.com/asset.png");
    assert_eq!(fetch_asset[1], "https://example.com/error");
}

/// Connect to a server, ensuring the protocol header is set, and return the client WS stream
pub async fn connect_client(
    addr: String,
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    let mut request = format!("ws://{addr}/")
        .into_client_request()
        .expect("Failed to build request");

    request.headers_mut().insert(
        "sec-websocket-protocol",
        HeaderValue::from_static(SUBPROTOCOL),
    );

    let (ws_stream, response) = tokio_tungstenite::connect_async(request)
        .await
        .expect("Failed to connect");

    assert_eq!(
        response.headers().get("sec-websocket-protocol"),
        Some(&HeaderValue::from_static(SUBPROTOCOL))
    );

    ws_stream
}
