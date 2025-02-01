#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PartialMetadata {
    pub sequence: Option<u32>,
    pub log_time: Option<u64>,
    pub publish_time: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Metadata {
    pub sequence: u32,
    pub log_time: u64,
    pub publish_time: u64,
}

impl PartialMetadata {
    pub const fn new() -> Self {
        Self {
            sequence: None,
            log_time: None,
            publish_time: None,
        }
    }
}
