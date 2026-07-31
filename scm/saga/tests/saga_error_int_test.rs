//! Integration tests for `SagaError`.

use edge_application_saga::SagaError;

#[test]
fn test_already_registered_display_contains_id_happy() {
    let msg = SagaError::AlreadyRegistered("proc-1".into()).to_string();
    assert!(msg.contains("proc-1"), "expected 'proc-1' in: {msg}");
}

#[test]
fn test_not_found_display_contains_id_error() {
    let msg = SagaError::NotFound("missing".into()).to_string();
    assert!(msg.contains("missing"), "expected 'missing' in: {msg}");
}

#[test]
fn test_errors_are_equality_comparable_edge() {
    let err1 = SagaError::AlreadyRegistered("x".into());
    let err2 = SagaError::AlreadyRegistered("x".into());
    assert_eq!(
        err1, err2,
        "identical AlreadyRegistered errors must be equal"
    );

    let notfound1 = SagaError::NotFound("y".into());
    let notfound2 = SagaError::NotFound("y".into());
    assert_eq!(
        notfound1, notfound2,
        "identical NotFound errors must be equal"
    );
    assert_ne!(
        err1,
        SagaError::NotFound("x".into()),
        "different error variants must not be equal"
    );
}
