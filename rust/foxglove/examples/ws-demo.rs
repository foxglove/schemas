use clap::Parser;

#[path = "common/lib.rs"]
mod common;

#[derive(Debug, Parser)]
struct Cli {
    /// Server TCP port.
    #[arg(short, long, default_value_t = 8765)]
    port: u16,
    /// Server IP address.
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    /// Frames per second.
    #[arg(long, default_value_t = 60)]
    fps: u8,
}

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let args = Cli::parse();

    let server = foxglove::WebSocketServer::new()
        .name("ws-demo")
        .bind(&args.host, args.port)
        .start()
        .await
        .expect("Server failed to start");

    tokio::task::spawn(common::log_forever(args.fps));
    tokio::signal::ctrl_c().await.ok();
    server.stop().await;
}
