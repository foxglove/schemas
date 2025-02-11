//! Example of a parameter server using the Foxglove SDK.
//!
//! This example uses the 'unstable' feature to expose capabilities.
//!
//! Usage:
//! ```text
//! cargo run -p example-param-server
//! ```

use std::time::{Duration, Instant};

use clap::Parser;
use foxglove::{
    Capability, Parameter, ParameterType, ParameterValue, WebSocketServer, WebSocketServerHandle,
};
use tokio_util::sync::CancellationToken;

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

    let args = Cli::parse();

    let server = WebSocketServer::new()
        .name("param server")
        .capabilities([Capability::Parameters])
        .bind(args.host, args.port)
        .start()
        .await
        .expect("Failed to start server");

    let shutdown = watch_ctrl_c();
    tokio::select! {
        () = shutdown.cancelled() => (),
        () = update_parameters(&server) => (),
    };

    server.stop().await;
}

async fn update_parameters(server: &WebSocketServerHandle) {
    let start = Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let parameter = Parameter {
            name: "elapsed".to_string(),
            value: Some(ParameterValue::Number(start.elapsed().as_secs_f64())),
            r#type: Some(ParameterType::Float64),
        };
        server.publish_parameter_values(vec![parameter]).await;
    }
}

fn watch_ctrl_c() -> CancellationToken {
    let token = CancellationToken::new();
    tokio::spawn({
        let token = token.clone();
        async move {
            tokio::signal::ctrl_c().await.ok();
            token.cancel();
        }
    });
    token
}
