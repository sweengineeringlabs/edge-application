//! SAF facade tests — `EventSource` trait via `ClosedEventSource`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{ClosedEventSource, EventError, EventSource, EventSourceRecvNextRequest};

/// @covers: ClosedEventSource::recv_next — returns Unavailable immediately
#[test]
fn test_recv_next_closed_source_returns_unavailable_happy() {
    let mut src = ClosedEventSource;
    let result = futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: ClosedEventSource — error message describes closed state
#[test]
fn test_recv_next_unavailable_message_is_descriptive_error() {
    let mut src = ClosedEventSource;
    let err = match futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest)) {
        Err(e) => e,
        Ok(_) => panic!("expected Err from closed source"),
    };
    let msg = err.to_string();
    assert!(!msg.is_empty(), "expected non-empty error message, got: {msg}");
}

/// @covers: ClosedEventSource — multiple calls all return Unavailable
#[test]
fn test_recv_next_repeated_calls_all_unavailable_edge() {
    let mut src = ClosedEventSource;
    for _ in 0..3 {
        assert!(matches!(
            futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest)),
            Err(EventError::Unavailable(_))
        ));
    }
}
