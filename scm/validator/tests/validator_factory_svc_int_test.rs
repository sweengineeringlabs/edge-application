//! SAF facade tests — `AlwaysValid` construction.

use edge_application_validator::{AlwaysValid, ValidationRequest, ValidationResponse, Validator};

/// @covers: AlwaysValid — returns a passing validator
#[test]
fn test_always_valid_returns_passing_validator_happy() {
    let v = AlwaysValid;
    assert_eq!(v.validate(ValidationRequest), Ok(ValidationResponse));
}

/// @covers: AlwaysValid — never rejects
#[test]
fn test_always_valid_never_rejects_error() {
    let v = AlwaysValid;
    assert_eq!(v.validate(ValidationRequest), Ok(ValidationResponse));
}

/// @covers: AlwaysValid — zero-sized marker
#[test]
fn test_always_valid_is_zero_size_edge() {
    let v = AlwaysValid;
    assert_eq!(std::mem::size_of_val(&v), 0);
}
