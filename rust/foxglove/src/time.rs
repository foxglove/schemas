use std::time::{SystemTime, UNIX_EPOCH};

pub fn nanoseconds_since_epoch() -> u64 {
    let now = SystemTime::now();
    if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
        return duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64;
    }
    0
}
