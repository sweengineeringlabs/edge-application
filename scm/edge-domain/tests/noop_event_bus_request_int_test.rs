//! Integration tests for `NoopEventBusRequest`.
#![cfg(feature = "event")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::NoopEventBusRequest;

/// @covers: NoopEventBusRequest
#[test]
fn test_noop_event_bus_request_default_happy() {
    let default: NoopEventBusRequest = Default::default();
    assert_eq!(default, NoopEventBusRequest);
}

/// @covers: NoopEventBusRequest
#[test]
fn test_noop_event_bus_request_debug_format_error() {
    assert_eq!(format!("{NoopEventBusRequest:?}"), "NoopEventBusRequest");
}

/// @covers: NoopEventBusRequest
#[test]
fn test_noop_event_bus_request_is_copy_edge() {
    let a = NoopEventBusRequest;
    let b = a;
    assert_eq!(a, b);
}
