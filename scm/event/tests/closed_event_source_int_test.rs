//! Integration tests for `ClosedEventSource`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{ClosedEventSource, EventError, EventSource, EventSourceRecvNextRequest};

/// @covers: ClosedEventSource — recv_next always returns Unavailable
#[test]
fn test_closed_event_source_recv_next_returns_unavailable_happy() {
    let mut src = ClosedEventSource;
    let result = futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: ClosedEventSource — error message is non-empty
#[test]
fn test_closed_event_source_error_message_non_empty_error() {
    let mut src = ClosedEventSource;
    let err = match futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest)) {
        Err(e) => e,
        Ok(_) => panic!("expected Err from ClosedEventSource"),
    };
    assert!(!err.to_string().is_empty());
}

/// @covers: ClosedEventSource — multiple calls all return Unavailable
#[test]
fn test_closed_event_source_repeated_calls_all_unavailable_edge() {
    let mut src = ClosedEventSource;
    for _ in 0..5 {
        assert!(matches!(
            futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest)),
            Err(EventError::Unavailable(_))
        ));
    }
}
