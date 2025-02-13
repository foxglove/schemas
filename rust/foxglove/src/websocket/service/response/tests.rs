use std::sync::atomic::Ordering;

use super::ResponseChannel;

#[tokio::test]
async fn test_response_channel() {
    let rc = ResponseChannel::new(3);
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 3);

    // Prepare some responses.
    let r1 = rc.prepare_response(1, 2, "raw").unwrap();
    let r2 = rc.prepare_response(2, 4, "raw").unwrap();
    let r3 = rc.prepare_response(3, 6, "raw").unwrap();

    // No more room.
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 0);
    assert!(rc.prepare_response(0, 0, "").is_none());

    // Complete a response. Still full until it's polled.
    r1.respond(Err("shucks".into()));
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 0);
    assert!(rc.prepare_response(0, 0, "").is_none());

    // Poll a response, now there's room again.
    let _ = rc.next_message().await;
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 1);
    let r4 = rc.prepare_response(4, 8, "").unwrap();
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 0);

    // Dropping a response immediately frees up room.
    drop(r2);
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 1);

    // Finish responding. There's still only one free slot until we poll.
    r3.respond(Err("oh noes".into()));
    r4.respond(Err("rats".into()));
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 1);

    // Draining the queue immediately frees up room.
    rc.drain();
    assert_eq!(rc.tokens.load(Ordering::Relaxed), 3);
}
