use assert_matches::assert_matches;
use tokio_tungstenite::tungstenite::Message;

use super::{send_lossy, SendLossyResult};

fn make_message(id: usize) -> Message {
    Message::Text(format!("{id}").into())
}

fn parse_message(msg: Message) -> usize {
    match msg {
        Message::Text(text) => text.parse().expect("id"),
        _ => unreachable!(),
    }
}

#[test]
fn test_send_lossy() {
    const BACKLOG: usize = 4;
    const TOTAL: usize = 10;

    let (tx, rx) = flume::bounded(BACKLOG);
    for i in 0..BACKLOG {
        assert_matches!(
            send_lossy(&tx, &rx, make_message(i), 0),
            SendLossyResult::Sent
        );
    }

    // The queue is full now. We'll only succeed with retries.
    for i in BACKLOG..TOTAL {
        assert_matches!(
            send_lossy(&tx, &rx, make_message(TOTAL + i), 0),
            SendLossyResult::ExhaustedRetries
        );
        assert_matches!(
            send_lossy(&tx, &rx, make_message(i), 1),
            SendLossyResult::SentLossy(1)
        );
    }

    // Receive everything, expect that the first (TOTAL - BACKLOG) messages were dropped.
    let mut received: Vec<usize> = vec![];
    while let Ok(msg) = rx.try_recv() {
        received.push(parse_message(msg));
    }
    assert_eq!(received, ((TOTAL - BACKLOG)..TOTAL).collect::<Vec<_>>());
}
