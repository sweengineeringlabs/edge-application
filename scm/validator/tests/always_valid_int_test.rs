//! Integration tests for the `AlwaysValid` reference validator.

use edge_application_validator::{AlwaysValid, ValidationRequest, ValidationResponse, Validator};

/// @covers: AlwaysValid (Validator::validate) — accepts by default
#[test]
fn test_validate_accepts_by_default_happy() {
    let result = AlwaysValid.validate(ValidationRequest);
    assert_eq!(result, Ok(ValidationResponse), "AlwaysValid should always succeed");
}

/// @covers: AlwaysValid — never rejects
#[test]
fn test_validate_never_returns_err_error() {
    let result = AlwaysValid.validate(ValidationRequest);
    assert_eq!(result, Ok(ValidationResponse), "should never return error");
}

/// @covers: AlwaysValid — usable via dyn dispatch
#[test]
fn test_validate_via_dyn_dispatch_edge() {
    let v: &dyn Validator = &AlwaysValid;
    let result = v.validate(ValidationRequest);
    assert_eq!(result, Ok(ValidationResponse), "dyn Validator should succeed");
}
