use crate::channel::{Channel, ChannelId};
use crate::log_sink::LogSink;
use crate::metadata::Metadata;
use crate::FoxgloveError;
use parking_lot::Mutex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

struct WriterState {
    writer: mcap::Writer<BufWriter<File>>,
    // ChannelId -> mcap file channel id
    channel_map: HashMap<ChannelId, u16>,
}

impl WriterState {
    fn new(writer: mcap::Writer<BufWriter<File>>) -> Self {
        Self {
            writer,
            channel_map: HashMap::new(),
        }
    }

    fn log(
        &mut self,
        channel: &Arc<Channel>,
        msg: &[u8],
        metadata: &Metadata,
    ) -> Result<(), FoxgloveError> {
        let channel_id = channel.id();
        let mcap_channel_id = match self.channel_map.entry(channel_id) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let Some(schema) = channel.schema.as_ref() else {
                    return Err(FoxgloveError::Fatal(
                        "Channel schema is required".to_string(),
                    ));
                };

                let schema_id = self
                    .writer
                    .add_schema(
                        &schema.name,
                        schema.encoding.as_deref().unwrap_or_default(),
                        &schema.data,
                    )
                    .map_err(FoxgloveError::from)?;

                let mcap_channel_id = self
                    .writer
                    .add_channel(
                        schema_id,
                        &channel.topic,
                        &channel.message_encoding,
                        &channel.metadata,
                    )
                    .map_err(FoxgloveError::from)?;

                entry.insert(mcap_channel_id);
                mcap_channel_id
            }
        };

        self.writer
            .write_to_known_channel(
                &mcap::records::MessageHeader {
                    channel_id: mcap_channel_id,
                    sequence: metadata.sequence,
                    log_time: metadata.log_time,
                    publish_time: metadata.publish_time,
                },
                msg,
            )
            .map_err(FoxgloveError::from)
    }
}

pub struct FileWriter(Mutex<Option<WriterState>>);

impl FileWriter {
    /// Open a file at the given path for writing.
    /// If the file is not writable or already exists, an error is returned.
    pub fn new(path: &Path) -> Result<Arc<FileWriter>, FoxgloveError> {
        let file = File::create_new(path).map_err(FoxgloveError::IOError)?;
        let mcap_writer = mcap::Writer::new(BufWriter::new(file)).map_err(FoxgloveError::from)?;
        let writer = Arc::new(Self(Mutex::new(Some(WriterState::new(mcap_writer)))));
        Ok(writer)
    }

    /// Flushes data to the file and closes it.
    pub fn close(&self) -> Result<(), FoxgloveError> {
        let Some(mut writer) = self.0.lock().take() else {
            return Ok(());
        };
        // TODO FG-9969 finish doesn't call close on the writer
        writer.writer.finish().map_err(FoxgloveError::from)
    }
}

impl LogSink for FileWriter {
    fn log(
        &self,
        channel: &Arc<Channel>,
        msg: &[u8],
        metadata: &Metadata,
    ) -> Result<(), FoxgloveError> {
        _ = metadata;
        let mut guard = self.0.lock();
        let writer = guard
            .as_mut()
            .ok_or(FoxgloveError::Fatal("File writer is closed".to_string()))?;

        writer.log(channel, msg, metadata)
    }

    fn close(&self) -> Result<(), FoxgloveError> {
        self.close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log_sink_set::LogSinkSet;
    use crate::{collection, Metadata, Schema};
    use mcap::McapError;
    use std::borrow::Cow;
    use std::fs::read_to_string;
    use std::io::Write;
    use std::sync::atomic::AtomicU32;
    use tempfile::NamedTempFile;

    fn new_test_channel(id: u64, topic: String, name: String) -> Arc<Channel> {
        Arc::new(Channel {
            sinks: LogSinkSet::new(),
            id: ChannelId::new(id),
            message_sequence: AtomicU32::new(1),
            topic,
            message_encoding: "message_encoding".to_string(),
            schema: Some(Schema {
                name,
                encoding: Some("encoding".to_string()),
                data: Cow::Borrowed(
                    br#"{
                    "type": "object",
                    "properties": {
                        "msg": {"type": "string"},
                        "count": {"type": "number"},
                    },
                }"#,
                ),
            }),
            metadata: collection! {"key".to_string() => "value".to_string()},
        })
    }

    fn foreach_mcap_message<F>(path: &Path, mut f: F) -> Result<(), McapError>
    where
        F: FnMut(mcap::Message),
    {
        let contents = std::fs::read(path).map_err(McapError::Io)?;
        let stream = mcap::MessageStream::new(&contents)?;
        for msg_result in stream {
            f(msg_result?);
        }
        Ok(())
    }

    #[test]
    fn test_writer_creates_file_if_needed() {
        // Generate a temporary file path without creating the file
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let temp_path = temp_dir.path().join("to-be-created.mcap");

        let writer = FileWriter::new(&temp_path).expect("failed to create writer");

        assert!(temp_path.exists());

        writer.close().expect("failed to close writer");
    }

    #[test]
    fn test_writer_errors_if_file_exists() {
        let mut temp_file = NamedTempFile::new().expect("failed to create temp file");

        temp_file
            .write_all("hello".as_bytes())
            .expect("failed to write to temp file");

        let temp_path = temp_file.path();

        let result = FileWriter::new(temp_path);
        assert!(result.is_err());

        // File was not truncated
        let contents = read_to_string(temp_file.path()).expect("failed to read temp file");
        assert_eq!(contents, "hello");
    }

    #[test]
    fn test_log_channels() {
        // Create two channels
        let ch1 = new_test_channel(1, "foo".to_string(), "foo_schema".to_string());
        let ch2 = new_test_channel(2, "bar".to_string(), "bar_schema".to_string());

        // Generate a temporary file path without creating the file
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let temp_path = temp_dir.path().join("test_log_channels.mcap");

        // Generate some unique metadata for each message
        let ch1_meta = &[
            Metadata {
                sequence: 1,
                publish_time: 2,
                log_time: 3,
            },
            Metadata {
                sequence: 4,
                publish_time: 5,
                log_time: 6,
            },
        ];
        let mut ch1_meta_iter = ch1_meta.iter();

        let ch2_meta = &[
            Metadata {
                sequence: 7,
                publish_time: 8,
                log_time: 9,
            },
            Metadata {
                sequence: 10,
                publish_time: 11,
                log_time: 12,
            },
        ];
        let mut ch2_meta_iter = ch2_meta.iter();

        // Log two messages to each channel, interleaved
        let writer = FileWriter::new(&temp_path).expect("failed to create writer");
        writer
            .log(&ch1, b"msg1", &ch1_meta[0])
            .expect("failed to log to channel 1");
        writer
            .log(&ch2, b"msg2", &ch2_meta[0])
            .expect("failed to log to channel 2");
        writer
            .log(&ch1, b"msg3", &ch1_meta[1])
            .expect("failed to log to channel 1");
        writer
            .log(&ch2, b"msg4", &ch2_meta[1])
            .expect("failed to log to channel 2");
        writer.close().expect("failed to close file");

        let ch1_msgs: &[&[u8]] = &[b"msg1", b"msg3"];
        let ch2_msgs: &[&[u8]] = &[b"msg2", b"msg4"];
        let mut ch1_msgs_iter = ch1_msgs.iter();
        let mut ch2_msgs_iter = ch2_msgs.iter();

        // Read the MCAP file and verify the contents
        foreach_mcap_message(&temp_path, |msg| {
            let channel_id = msg.channel.id;
            let payload = msg.data;
            match channel_id {
                0 => {
                    assert_eq!(
                        &payload,
                        ch1_msgs_iter.next().expect("unexpected message channel 1")
                    );
                    let metadata = ch1_meta_iter.next().expect("unexpected metadata channel 1");
                    assert_eq!(msg.sequence, metadata.sequence);
                    assert_eq!(msg.publish_time, metadata.publish_time);
                    assert_eq!(msg.log_time, metadata.log_time);
                    assert_eq!(msg.channel.topic, "foo");
                    assert_eq!(
                        msg.channel.schema.as_ref().expect("missing schema").name,
                        "foo_schema"
                    );
                }
                1 => {
                    assert_eq!(
                        &payload,
                        ch2_msgs_iter.next().expect("unexpected message channel 2")
                    );
                    let metadata = ch2_meta_iter.next().expect("unexpected metadata channel 2");
                    assert_eq!(msg.sequence, metadata.sequence);
                    assert_eq!(msg.publish_time, metadata.publish_time);
                    assert_eq!(msg.log_time, metadata.log_time);
                    assert_eq!(msg.channel.topic, "bar");
                    assert_eq!(
                        msg.channel.schema.as_ref().expect("missing schema").name,
                        "bar_schema"
                    );
                }
                _ => panic!("unexpected channel id: {}", channel_id),
            }
        })
        .expect("failed to read MCAP messages");
    }
}
