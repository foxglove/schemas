//! Example of a parameter server using the Foxglove SDK.
//!
//! This example uses the 'unstable' feature to expose capabilities.
//!
//! Usage:
//! ```text
//! cargo run --features unstable --example param-server
//! ```
use std::{process::exit, time};

use clap::Parser;
use foxglove::websocket::{
    create_server_with_internal_options, Capability, InternalServerOptions, Parameter,
    ParameterType, ParameterValue,
};
use std::collections::HashSet;
use tokio::{
    signal, spawn,
    time::{sleep, Duration},
};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 8765)]
    port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}
#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let server = create_server_with_internal_options(InternalServerOptions {
        capabilities: Some(HashSet::from([Capability::Parameters])),
        session_id: None,
        name: Some("Example param server".to_string()),
        listener: None,
        message_backlog_size: None,
        supported_encodings: None,
    });
    let args = Cli::parse();

    match server.start(&args.host, args.port).await {
        Ok(addr) => tracing::info!("Server started at {addr}"),
        Err(e) => {
            tracing::error!("Server start error: {e}");
            exit(1);
        }
    };

    spawn(async move {
        signal::ctrl_c()
            .await
            .expect("Failed to wait for interrupt");
        exit(0);
    });

    let start = time::Instant::now();
    loop {
        let parameter = Parameter {
            name: "elapsed".to_string(),
            value: Some(ParameterValue::Number(start.elapsed().as_secs_f64())),
            r#type: Some(ParameterType::Float64),
        };
        server.publish_parameter_values(vec![parameter], None).await;
        sleep(Duration::from_secs(1)).await;
    }
}
