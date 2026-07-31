//! Integration tests for `ValidationResponse`.

use edge_application_base::ValidationResponse;

/// @covers: ValidationResponse — is zero-sized and constructible
#[test]
fn test_validation_response_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ValidationResponse>(), 0);
    let _ = ValidationResponse;
}

/// @covers: ValidationResponse — implements Default, matching a directly-constructed instance
#[test]
fn test_validation_response_default_matches_direct_construction_edge() {
    let direct = ValidationResponse;
    let via_default: ValidationResponse = Default::default();
    assert_eq!(direct, via_default);
}
