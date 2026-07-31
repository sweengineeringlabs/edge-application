//! Integration tests — `HandlerError` display and helper constructors.

use edge_application_handler::HandlerError;

/// @covers: HandlerError::Unsupported display
#[test]
fn test_display_unsupported_variant_happy() {
    let e = HandlerError::Unsupported("op".into());
    assert_eq!(e.to_string(), "unsupported operation: op");
}

/// @covers: HandlerError::InvalidRequest display
#[test]
fn test_display_invalid_request_variant_happy() {
    let e = HandlerError::InvalidRequest("bad".into());
    assert_eq!(e.to_string(), "invalid request: bad");
}

/// @covers: HandlerError::NotFound display
#[test]
fn test_display_not_found_variant_happy() {
    let e = HandlerError::NotFound("item".into());
    assert_eq!(e.to_string(), "not found: item");
}

/// @covers: HandlerError::Conflict display
#[test]
fn test_display_conflict_variant_happy() {
    let e = HandlerError::Conflict("state".into());
    assert_eq!(e.to_string(), "conflict: state");
}

/// @covers: HandlerError::ExecutionFailed display
#[test]
fn test_display_execution_failed_variant_happy() {
    let e = HandlerError::ExecutionFailed("oops".into());
    assert_eq!(e.to_string(), "execution failed: oops");
}

/// @covers: HandlerError::Unhealthy display
#[test]
fn test_display_unhealthy_variant_happy() {
    let e = HandlerError::Unhealthy;
    assert_eq!(e.to_string(), "handler unhealthy");
}

/// @covers: HandlerError::FailedPrecondition display
#[test]
fn test_display_failed_precondition_variant_happy() {
    let e = HandlerError::FailedPrecondition("pre".into());
    assert_eq!(e.to_string(), "failed precondition: pre");
}

/// @covers: HandlerError::Unauthorized display
#[test]
fn test_display_unauthorized_variant_happy() {
    let e = HandlerError::Unauthorized("who".into());
    assert_eq!(e.to_string(), "unauthorized: who");
}

/// @covers: HandlerError::PermissionDenied display
#[test]
fn test_display_permission_denied_variant_happy() {
    let e = HandlerError::PermissionDenied("action".into());
    assert_eq!(e.to_string(), "permission denied: action");
}

/// @covers: HandlerError::Timeout display
#[test]
fn test_display_timeout_variant_happy() {
    let e = HandlerError::Timeout("30s".into());
    assert_eq!(e.to_string(), "timeout: 30s");
}

/// @covers: HandlerError::Skipped display
#[test]
fn test_display_skipped_variant_happy() {
    let e = HandlerError::Skipped;
    assert_eq!(e.to_string(), "handler skipped");
}

/// @covers: HandlerError::ExecutionFailed — wraps execution failure message
#[test]
fn test_execution_failed_variant_wraps_message_happy() {
    let e = HandlerError::ExecutionFailed("db down".into());
    assert!(matches!(e, HandlerError::ExecutionFailed(_)));
    assert_eq!(e.to_string(), "execution failed: db down");
}

/// @covers: HandlerError::InvalidRequest — wraps invalid input message
#[test]
fn test_invalid_request_variant_wraps_message_happy() {
    let e = HandlerError::InvalidRequest("null pointer".into());
    assert!(matches!(e, HandlerError::InvalidRequest(_)));
    assert_eq!(e.to_string(), "invalid request: null pointer");
}

/// @covers: HandlerError::ExecutionFailed — empty message produces variant
#[test]
fn test_execution_failed_empty_message_produces_variant_edge() {
    let e = HandlerError::ExecutionFailed(String::new());
    assert_eq!(e.to_string(), "execution failed: ");
}
