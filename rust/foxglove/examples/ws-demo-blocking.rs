//! A websocket server without a #[tokio::main] entrypoint.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

fn main() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let args = Cli::parse();

    let done = Arc::new(AtomicBool::default());
    ctrlc::set_handler({
        let done = done.clone();
        move || {
            done.store(true, Ordering::Relaxed);
        }
    })
    .expect("Failed to set SIGINT handler");

    let server = foxglove::WebSocketServer::new()
        .name("ws-demo")
        .bind(&args.host, args.port)
        .start_blocking()
        .expect("Server failed to start");

    common::log_blocking(args.fps, done);
    server.stop();
}
