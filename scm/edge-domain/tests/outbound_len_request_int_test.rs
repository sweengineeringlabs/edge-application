//! Integration tests for `OutboundLenRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::OutboundLenRequest;

/// @covers: OutboundLenRequest
#[test]
fn test_outbound_len_request_default_happy() {
    let default: OutboundLenRequest = Default::default();
    assert_eq!(default, OutboundLenRequest);
}

/// @covers: OutboundLenRequest
#[test]
fn test_outbound_len_request_debug_format_error() {
    assert_eq!(format!("{OutboundLenRequest:?}"), "OutboundLenRequest");
}

/// @covers: OutboundLenRequest
#[test]
fn test_outbound_len_request_is_copy_edge() {
    let a = OutboundLenRequest;
    let b = a;
    assert_eq!(a, b);
}
