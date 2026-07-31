//! Integration tests for `EventError`.

use edge_application_event::EventError;

/// @covers: EventError — Unavailable variant displays correctly
#[test]
fn test_event_error_unavailable_display_contains_message_happy() {
    let err = EventError::Unavailable("bus closed".into());
    let msg = err.to_string();
    assert!(msg.contains("bus closed"), "unexpected display: {msg}");
}

/// @covers: EventError — SerializationFailed variant displays correctly
#[test]
fn test_event_error_serialization_failed_display_contains_reason_error() {
    let err = EventError::SerializationFailed("invalid json".into());
    let msg = err.to_string();
    assert!(msg.contains("invalid json"), "unexpected display: {msg}");
}

/// @covers: EventError — BroadcastLagged variant displays dropped count
#[test]
fn test_event_error_broadcast_lagged_display_contains_count_edge() {
    let err = EventError::BroadcastLagged(42);
    let msg = err.to_string();
    assert!(msg.contains("42"), "unexpected display: {msg}");
}
