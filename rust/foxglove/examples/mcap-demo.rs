use std::path::PathBuf;

use clap::Parser;

#[path = "common/lib.rs"]
mod common;

#[derive(Debug, Parser)]
struct Cli {
    /// Output path.
    #[arg(short, long, default_value = "output.mcap")]
    path: PathBuf,
    /// Frames per second.
    #[arg(long, default_value_t = 10)]
    fps: u8,
}

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let args = Cli::parse();

    let writer = foxglove::McapWriter::new(&args.path)
        .create()
        .expect("Failed to start mcap writer");

    tokio::task::spawn(common::log_forever(args.fps));
    tokio::signal::ctrl_c().await.ok();
    writer.close().expect("Failed to flush mcap file");
}
