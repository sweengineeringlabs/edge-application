//! Integration tests for `ProjectionError`.

use edge_application_projection::ProjectionError;

#[test]
fn test_empty_stream_display_contains_empty_happy() {
    let msg = ProjectionError::EmptyStream.to_string();
    assert!(msg.contains("empty"), "expected 'empty' in: {msg}");
}

#[test]
fn test_internal_error_display_contains_message_error() {
    let msg = ProjectionError::Internal("bad state".into()).to_string();
    assert!(msg.contains("bad state"), "expected 'bad state' in: {msg}");
}

#[test]
fn test_errors_are_equality_comparable_edge() {
    let err1 = ProjectionError::EmptyStream;
    let err2 = ProjectionError::EmptyStream;
    assert_eq!(err1, err2, "same variant must be equal");

    let internal1 = ProjectionError::Internal("x".into());
    let internal2 = ProjectionError::Internal("x".into());
    assert_eq!(
        internal1,
        internal2,
        "identical internal errors must be equal"
    );
    assert_ne!(
        err1,
        internal1,
        "different variants must not be equal"
    );
}
