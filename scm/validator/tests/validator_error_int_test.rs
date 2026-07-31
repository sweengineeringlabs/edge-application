//! Tests for the `ValidatorError` error type.

use edge_application_validator::ValidatorError;

/// @covers: ValidatorError::Invalid — Display includes message
#[test]
fn test_invalid_display_includes_message_happy() {
    let e = ValidatorError::Invalid("bad range".into());
    assert!(e.to_string().contains("bad range"));
}

/// @covers: ValidatorError::Invalid — equality by value
#[test]
fn test_invalid_equality_by_value_error() {
    let err1 = ValidatorError::Invalid("x".into());
    let err2 = ValidatorError::Invalid("x".into());
    assert_eq!(
        err1,
        err2,
        "errors with same message must be equal"
    );
}

/// @covers: ValidatorError::Invalid — distinct messages differ
#[test]
fn test_invalid_distinct_messages_differ_edge() {
    let err_a = ValidatorError::Invalid("a".into());
    let err_b = ValidatorError::Invalid("b".into());
    assert_ne!(err_a, err_b, "errors with different messages must not be equal");
    assert_eq!(err_a, ValidatorError::Invalid("a".into()), "same messages must be equal");
}
