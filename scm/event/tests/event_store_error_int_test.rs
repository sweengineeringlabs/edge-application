//! Integration tests for `EventStoreError`.

use edge_application_event::EventStoreError;

/// @covers: EventStoreError — Conflict variant displays aggregate id and versions
#[test]
fn test_event_store_error_conflict_display_contains_ids_happy() {
    let err = EventStoreError::Conflict {
        aggregate_id: "order-1".into(),
        expected: 0,
        actual: 5,
    };
    let msg = err.to_string();
    assert!(msg.contains("order-1"), "expected aggregate_id in: {msg}");
    assert!(msg.contains('0'.to_string().as_str()), "expected expected version in: {msg}");
    assert!(msg.contains('5'.to_string().as_str()), "expected actual version in: {msg}");
}

/// @covers: EventStoreError — Unavailable variant displays reason
#[test]
fn test_event_store_error_unavailable_display_contains_reason_error() {
    let err = EventStoreError::Unavailable("store offline".into());
    let msg = err.to_string();
    assert!(msg.contains("store offline"), "unexpected display: {msg}");
}

/// @covers: EventStoreError — Internal variant displays message
#[test]
fn test_event_store_error_internal_display_contains_message_edge() {
    let err = EventStoreError::Internal("unexpected panic".into());
    let msg = err.to_string();
    assert!(msg.contains("unexpected panic"), "unexpected display: {msg}");
}
