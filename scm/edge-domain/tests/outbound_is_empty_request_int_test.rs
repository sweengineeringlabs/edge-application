//! Integration tests for `OutboundIsEmptyRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::OutboundIsEmptyRequest;

/// @covers: OutboundIsEmptyRequest
#[test]
fn test_outbound_is_empty_request_default_happy() {
    let default: OutboundIsEmptyRequest = Default::default();
    assert_eq!(default, OutboundIsEmptyRequest);
}

/// @covers: OutboundIsEmptyRequest
#[test]
fn test_outbound_is_empty_request_debug_format_error() {
    assert_eq!(
        format!("{OutboundIsEmptyRequest:?}"),
        "OutboundIsEmptyRequest"
    );
}

/// @covers: OutboundIsEmptyRequest
#[test]
fn test_outbound_is_empty_request_is_copy_edge() {
    let a = OutboundIsEmptyRequest;
    let b = a;
    assert_eq!(a, b);
}
