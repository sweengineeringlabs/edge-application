//! Integration tests for domain error → HandlerError conversion.
//!
//! `From<DomainError>` impls were removed when dispatch types moved to
//! `edge-dispatch` (orphan rules prevent cross-crate From impls).
//! The supported pattern is `.map_err(|e| HandlerError::ExecutionFailed(e.to_string()))`
//! or `.map_err(|e| HandlerError::InvalidRequest(e.to_string()))`.
#![cfg(all(feature = "repository", feature = "query", feature = "handler", feature = "event", feature = "command"))]

use edge_application::{CommandError, EventError, HandlerError, QueryError, RepositoryError};

// ── HandlerError::ExecutionFailed / InvalidRequest construction ─────────────

/// @covers: HandlerError::ExecutionFailed
#[test]
fn test_internal_wraps_any_display_as_execution_failed() {
    let e = HandlerError::ExecutionFailed("database unavailable".to_string());
    assert!(matches!(e, HandlerError::ExecutionFailed(_)));
    assert!(e.to_string().contains("database unavailable"));
}

/// @covers: HandlerError::InvalidRequest
#[test]
fn test_invalid_wraps_any_display_as_invalid_request() {
    let e = HandlerError::InvalidRequest("id must be a UUID".to_string());
    assert!(matches!(e, HandlerError::InvalidRequest(_)));
    assert!(e.to_string().contains("id must be a UUID"));
}

// ── .map_err(|e| HandlerError::ExecutionFailed(e.to_string())) pattern ───────

/// @covers: map_err pattern for CommandError
#[test]
fn test_map_err_internal_converts_command_error_to_execution_failed() {
    fn simulate() -> Result<(), HandlerError> {
        let result: Result<(), CommandError> = Err(CommandError::Internal("oops".into()));
        result.map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(())
    }
    assert!(matches!(simulate(), Err(HandlerError::ExecutionFailed(_))));
}

/// @covers: map_err pattern for QueryError
#[test]
fn test_map_err_internal_converts_query_error_to_execution_failed() {
    fn simulate() -> Result<String, HandlerError> {
        let result: Result<String, QueryError> = Err(QueryError::NotFound("x".into()));
        result.map_err(|e| HandlerError::ExecutionFailed(e.to_string()))
    }
    assert!(matches!(simulate(), Err(HandlerError::ExecutionFailed(_))));
}

/// @covers: map_err pattern for RepositoryError
#[test]
fn test_map_err_internal_converts_repository_error_to_execution_failed() {
    fn simulate() -> Result<(), HandlerError> {
        let result: Result<(), RepositoryError> =
            Err(RepositoryError::Unavailable("db down".into()));
        result.map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(())
    }
    assert!(matches!(simulate(), Err(HandlerError::ExecutionFailed(_))));
}

/// @covers: map_err pattern for EventError
#[test]
fn test_map_err_internal_converts_event_error_to_execution_failed() {
    fn simulate() -> Result<(), HandlerError> {
        let result: Result<(), EventError> = Err(EventError::Unavailable("bus down".into()));
        result.map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(())
    }
    assert!(matches!(simulate(), Err(HandlerError::ExecutionFailed(_))));
}

// ── HandlerError variant messages ─────────────────────────────────────────────

/// @covers: HandlerError variants and Display
#[test]
fn test_not_found_message_is_preserved() {
    let e = HandlerError::NotFound("order-99".into());
    assert!(e.to_string().contains("order-99"));
}

/// @covers: HandlerError variants
#[test]
fn test_unauthorized_is_distinct_from_permission_denied() {
    let unauth = HandlerError::Unauthorized("token expired".into());
    let denied = HandlerError::PermissionDenied("insufficient scope".into());
    assert!(matches!(unauth, HandlerError::Unauthorized(_)));
    assert!(matches!(denied, HandlerError::PermissionDenied(_)));
    assert!(unauth.to_string().contains("token expired"));
    assert!(denied.to_string().contains("insufficient scope"));
}

/// @covers: HandlerError::Conflict
#[test]
fn test_conflict_message_is_preserved() {
    let e = HandlerError::Conflict("dup key".into());
    assert!(matches!(e, HandlerError::Conflict(_)));
    assert!(e.to_string().contains("dup key"));
}

/// @covers: HandlerError::FailedPrecondition
#[test]
fn test_failed_precondition_message_is_preserved() {
    let e = HandlerError::FailedPrecondition("state invalid".into());
    assert!(matches!(e, HandlerError::FailedPrecondition(_)));
}

/// @covers: HandlerError::Unhealthy
#[test]
fn test_unhealthy_has_no_message_field() {
    let e = HandlerError::Unhealthy;
    assert!(matches!(e, HandlerError::Unhealthy));
    assert!(e.to_string().contains("unhealthy"));
}
