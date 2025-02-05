use std::{collections::HashSet, sync::Arc};

use crate::testutil::RecordingServerListener;
use crate::websocket::{
    create_server, Capability, ClientChannelId, Parameter, ParameterType, ParameterValue,
    ServerOptions, SUBPROTOCOL,
};
use bytes::{BufMut, BytesMut};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue, Message};

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

    // Give the server time to process the messages
    sleep(std::time::Duration::from_millis(10)).await;

    // Server should have received one message
    let mut received = recording_listener.take_message_data();
    let (channel_id, payload) = received.pop().expect("No message received");
    assert_eq!(channel_id, ClientChannelId::new(1));
    assert_eq!(payload, b"{\"a\":1}");

    // Server should have ignored the duplicate advertisement
    let advertisements = recording_listener.take_client_advertise();
    assert_eq!(advertisements.len(), 1);
    assert_eq!(advertisements[0].id, channel_id);

    // Server should have received one unadvertise (and ignored the duplicate)
    let unadvertises = recording_listener.take_client_unadvertise();
    assert_eq!(unadvertises.len(), 1);
    assert_eq!(unadvertises[0], channel_id);

    ws_client.close(None).await.unwrap();
    server.stop().await;
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
