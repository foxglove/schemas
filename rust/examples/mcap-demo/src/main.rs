use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use foxglove::McapWriter;
use mcap::{Compression, WriteOptions};
use std::time::Duration;

#[derive(Debug, Parser)]
struct Cli {
    /// Output path.
    #[arg(short, long, default_value = "output.mcap")]
    path: PathBuf,
    /// If set, overwrite an existing file.
    #[arg(long)]
    overwrite: bool,
    /// Chunk size.
    #[arg(long, default_value_t = 1024 * 768)]
    chunk_size: u64,
    /// Compression algorithm to use.
    #[arg(long, default_value = "zstd")]
    compression: CompressionArg,
    /// Frames per second.
    #[arg(long, default_value_t = 10)]
    fps: u8,
}

#[derive(Debug, Clone, ValueEnum)]
enum CompressionArg {
    Zstd,
    Lz4,
    None,
}
impl From<CompressionArg> for Option<Compression> {
    fn from(value: CompressionArg) -> Self {
        match value {
            CompressionArg::Zstd => Some(Compression::Zstd),
            CompressionArg::Lz4 => Some(Compression::Lz4),
            CompressionArg::None => None,
        }
    }
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
struct Message {
    msg: String,
    count: u32,
}

foxglove::static_typed_channel!(pub MSG_CHANNEL, "/msg", Message);

pub async fn log_forever(fps: u8) {
    let mut counter: u32 = 0;
    let mut interval = tokio::time::interval(Duration::from_millis(1000 / u64::from(fps)));
    loop {
        interval.tick().await;
        MSG_CHANNEL.log(&Message {
            msg: "Hello, world!".to_string(),
            count: counter,
        });
        counter += 1;
    }
}

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let args = Cli::parse();

    if args.overwrite && args.path.exists() {
        std::fs::remove_file(&args.path).expect("Failed to remove file");
    }

    let options = WriteOptions::new()
        .chunk_size(Some(args.chunk_size))
        .compression(args.compression.into());

    let writer = McapWriter::with_options(options)
        .create_new_buffered_file(&args.path)
        .expect("Failed to start mcap writer");

    tokio::task::spawn(log_forever(args.fps));
    tokio::signal::ctrl_c().await.ok();
    writer.close().expect("Failed to flush mcap file");
}
