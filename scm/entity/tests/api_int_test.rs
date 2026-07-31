//! Layer-level coverage for the small request/response value types declared under
//! `api/entity/dto/` that have no dedicated per-type test file (SEA layer test
//! coverage, `sea_layer_test_coverage`). Each test constructs the type through the
//! crate's public API and asserts on its real shape or field values.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_entity::{IdRequest, IdResponse, ValidationRequest, ValidationResponse};

/// @covers: IdRequest
#[test]
fn test_id_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<IdRequest>(), 0);
    let _ = IdRequest;
}

/// @covers: ValidationRequest
#[test]
fn test_validation_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ValidationRequest>(), 0);
    let _ = ValidationRequest;
}

/// @covers: ValidationResponse
#[test]
fn test_validation_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ValidationResponse>(), 0);
    let _ = ValidationResponse;
}

/// @covers: IdResponse
#[test]
fn test_id_response_holds_reference_to_id_happy() {
    let id = 42u64;
    let r = IdResponse { id: &id };
    assert_eq!(*r.id, 42u64);
}
