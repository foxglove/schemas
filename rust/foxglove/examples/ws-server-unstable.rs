//! Example websocket server with client publsih
//!
//! This example uses the 'unstable' feature to expose capabilities.
//!
//! Usage:
//! ```text
//! cargo run --features unstable --example ws-server
//! ```

use clap::Parser;
use foxglove::websocket::{self, create_server_with_internal_options, Capability, ServerListener};
use foxglove::{nanoseconds_since_epoch, ChannelBuilder, LogSink, Metadata, Schema};
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{self, Duration};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 8765)]
    port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value = "example-json-server")]
    server_name: String,
}

struct ExampleCallbackHandler;
impl ServerListener for ExampleCallbackHandler {
    fn on_message_data(&self, channel_id: websocket::ClientChannelId, message: &[u8]) {
        let json: serde_json::Value =
            serde_json::from_slice(message).expect("Failed to parse message");
        println!("Received message on channel {channel_id}: {json}");
    }
}

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    let args = Cli::parse();

    let session_id = new_session_id();
    let server = create_server_with_internal_options(websocket::InternalServerOptions {
        session_id: Some(session_id),
        name: Some(args.server_name),
        listener: Some(Arc::new(ExampleCallbackHandler)),
        capabilities: Some(HashSet::from([Capability::ClientPublish])),
        supported_encodings: Some(HashSet::from(["json".to_string()])),
        message_backlog_size: None,
    });

    match server.start(&args.host, args.port).await {
        Ok(addr) => tracing::info!("Server started at {addr}"),
        Err(e) => {
            tracing::error!("Server start error: {e}");
            std::process::exit(1);
        }
    };

    let async_task = async {
        let channel = ChannelBuilder::new("topic")
            .message_encoding("json")
            .schema(Schema::new(
                "schema_name",
                "jsonschema",
                br#"{"$schema":"http://json-schema.org/draft-07/schema#","type":"object","required":["msg","count"],"properties":{"msg":{"type":"string","description":"A message string"},"count":{"type":"number","description":"A numeric count value"}},"additionalProperties":false}"#,
            ))
            .add_metadata("key", "value")
            .build()
            .expect("Failed to create channel");

        server.add_channel(&channel);

        let mut count: u32 = 0;
        let mut timestamp = nanoseconds_since_epoch();
        let metadata = Metadata {
            sequence: channel.next_sequence(),
            publish_time: timestamp,
            log_time: 0,
        };
        loop {
            count += 1;
            let payload = json!({
                "msg": "Hello!",
                "count": count,
            })
            .to_string();
            server.log(&channel, payload.as_bytes(), &metadata).unwrap();

            tokio::time::sleep(Duration::from_millis(50)).await;
            timestamp += 50_000_000;
        }
    };

    tokio::select! {
        _ = tokio::signal::ctrl_c() => server.stop().await,
        _ = async_task => (),
    }
}

fn new_session_id() -> String {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Failed to create session ID; invalid system time")
        .as_millis()
        .to_string()
}
