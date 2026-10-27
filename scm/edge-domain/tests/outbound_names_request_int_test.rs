//! Integration tests for `OutboundNamesRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::OutboundNamesRequest;

/// @covers: OutboundNamesRequest
#[test]
fn test_outbound_names_request_default_happy() {
    let default: OutboundNamesRequest = Default::default();
    assert_eq!(default, OutboundNamesRequest);
}

/// @covers: OutboundNamesRequest
#[test]
fn test_outbound_names_request_debug_format_error() {
    assert_eq!(format!("{OutboundNamesRequest:?}"), "OutboundNamesRequest");
}

/// @covers: OutboundNamesRequest
#[test]
fn test_outbound_names_request_is_copy_edge() {
    let a = OutboundNamesRequest;
    let b = a;
    assert_eq!(a, b);
}
