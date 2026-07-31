//! Layer-level coverage for `api/valueobject/dto/*.rs` request types.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::default_constructed_unit_structs
)]

use edge_application_valueobject::ValidationRequest;

/// @covers: ValidationRequest
#[test]
fn test_validation_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ValidationRequest>(), 0);
    let _ = ValidationRequest;
}

/// @covers: ValidationRequest
#[test]
fn test_validation_request_default_equals_literal_happy() {
    assert_eq!(ValidationRequest::default(), ValidationRequest);
}
