use futures_util::{FutureExt, SinkExt, StreamExt};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio_tungstenite::tungstenite::{self, http::HeaderValue, Message};
use tungstenite::client::IntoClientRequest;

use crate::websocket::{
    create_server, ClientMessage, ServerOptions, StatusLevel, SubscriptionId, SUBPROTOCOL,
};
use crate::{collection, Channel, ChannelBuilder, LogContext, LogSink, Metadata, Schema};

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

#[tokio::test]
async fn test_advertise_to_client() {
    let server = create_server(ServerOptions::default());

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

    // Allow the server to process the subscription
    // FG-9723: replace this with an on_subscribe callback
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    server.log(&ch, b"{\"a\":1}", &metadata).unwrap();

    let result = client_receiver.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let data = msg.into_data();

    assert_eq!(data[0], 0x01); // message data opcode
    assert_eq!(
        u32::from_le_bytes(data[1..=4].try_into().unwrap()),
        subscription_id
    );

    server.stop().await;
}

#[tokio::test]
async fn test_log_only_to_subscribers() {
    let server = create_server(ServerOptions::default());

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

    let unsubscribe_both = json!(ClientMessage::Unsubscribe {
        subscription_ids: vec![SubscriptionId::new(1), SubscriptionId::new(2)]
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
    // FG-9723: replace this with an on_subscribe callback
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

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

#[tokio::test]
async fn test_error_status_message() {
    let server = create_server(ServerOptions::default());
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    _ = ws_client.next().await.expect("No serverInfo sent");

    ws_client
        .send(Message::text("nonsense".to_string()))
        .await
        .expect("Failed to send message");

    let result = ws_client.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let status: Value = serde_json::from_str(&text).expect("Failed to parse status");
    assert_eq!(status["level"], 2);
    assert_eq!(status["message"], "Unsupported message: nonsense");

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

    ws_client
        .send(Message::binary(vec![0xff]))
        .await
        .expect("Failed to send message");

    let result = ws_client.next().await.unwrap();
    let msg = result.expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    let status: Value = serde_json::from_str(&text).expect("Failed to parse status");
    assert_eq!(status["level"], 2);
    assert_eq!(status["message"], "Invalid binary opcode: 255");

    server.stop().await;
}

#[tokio::test]
async fn test_publish_status_message() {
    let server = create_server(ServerOptions::default());
    let addr = server
        .start("127.0.0.1", 0)
        .await
        .expect("Failed to start server");

    let mut ws_client = connect_client(addr).await;

    _ = ws_client.next().await.expect("No serverInfo sent");

    server.publish_status(
        StatusLevel::Info,
        "Hello, world!".to_string(),
        Some("123".to_string()),
    );
    server.publish_status(
        StatusLevel::Error,
        "Reactor core overload!".to_string(),
        Some("abc".to_string()),
    );

    let msg = ws_client
        .next()
        .await
        .expect("No message received")
        .expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    assert_eq!(
        text,
        r#"{"op":"status","level":1,"message":"Hello, world!","id":"123"}"#
    );

    let msg = ws_client
        .next()
        .await
        .expect("No message received")
        .expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    assert_eq!(
        text,
        r#"{"op":"status","level":3,"message":"Reactor core overload!","id":"abc"}"#
    );
}

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
    assert_eq!(text, r#"{"op":"removeStatus","statusIds":["123","abc"]}"#);

    let msg = ws_client2
        .next()
        .await
        .expect("No message received")
        .expect("Failed to parse message");
    let text = msg.into_text().expect("Failed to get message text");
    assert_eq!(text, r#"{"op":"removeStatus","statusIds":["123","abc"]}"#);
}

/// Connect to a server, ensuring the protocol header is set, and return the client WS stream
async fn connect_client(
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
