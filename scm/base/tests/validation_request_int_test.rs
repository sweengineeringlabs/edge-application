//! Integration tests for `ValidationRequest`.

use edge_application_base::ValidationRequest;

/// @covers: ValidationRequest — is zero-sized and constructible
#[test]
fn test_validation_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ValidationRequest>(), 0);
    let _ = ValidationRequest;
}

/// @covers: ValidationRequest — implements Default, matching a directly-constructed instance
#[test]
fn test_validation_request_default_matches_direct_construction_edge() {
    let direct = ValidationRequest;
    let via_default: ValidationRequest = Default::default();
    assert_eq!(direct, via_default);
}
