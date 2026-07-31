//! Integration tests — `Validator` trait via `AlwaysValid`.

use edge_application_validator::{AlwaysValid, ValidationRequest, ValidationResponse, Validator};

/// @covers: Validator::validate — always-valid reference impl
#[test]
fn test_validate_always_valid_returns_ok_happy() {
    assert_eq!(
        AlwaysValid.validate(ValidationRequest),
        Ok(ValidationResponse)
    );
}

/// @covers: Validator::validate — infallible for AlwaysValid
#[test]
fn test_validate_always_valid_is_idempotent_edge() {
    for _ in 0..3 {
        assert_eq!(AlwaysValid.validate(ValidationRequest), Ok(ValidationResponse));
    }
}
