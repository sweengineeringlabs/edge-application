//! Integration tests for `NoopEventPublisherRequest`.
#![cfg(feature = "event")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::NoopEventPublisherRequest;

/// @covers: NoopEventPublisherRequest
#[test]
fn test_noop_event_publisher_request_default_happy() {
    let default: NoopEventPublisherRequest = Default::default();
    assert_eq!(default, NoopEventPublisherRequest,);
}

/// @covers: NoopEventPublisherRequest
#[test]
fn test_noop_event_publisher_request_debug_format_error() {
    assert_eq!(
        format!("{NoopEventPublisherRequest:?}"),
        "NoopEventPublisherRequest"
    );
}

/// @covers: NoopEventPublisherRequest
#[test]
fn test_noop_event_publisher_request_is_copy_edge() {
    let a = NoopEventPublisherRequest;
    let b = a;
    assert_eq!(a, b);
}
