//! Tests for configuration and error types across api/ modules.
#![cfg(all(feature = "repository", feature = "query", feature = "handler", feature = "event"))]

use edge_application::{EventError, HandlerError, QueryError, RepositoryError};

/// @covers: HandlerError::ExecutionFailed
#[test]
fn test_handler_error_internal() {
    let err = HandlerError::ExecutionFailed("test error".to_string());
    assert!(err.to_string().contains("test error"));
}

/// @covers: HandlerError::NotFound
#[test]
fn test_handler_error_not_found() {
    let err = HandlerError::NotFound("missing".to_string());
    assert!(err.to_string().contains("missing"));
}

/// @covers: RepositoryError::NotFound
#[test]
fn test_repository_error_not_found() {
    let err = RepositoryError::NotFound("entity".to_string());
    assert!(err.to_string().contains("entity"));
}

/// @covers: QueryError::InvalidInput
#[test]
fn test_query_error_invalid_input() {
    let err = QueryError::InvalidInput("query failed".to_string());
    assert!(err.to_string().contains("query failed"));
}

/// @covers: EventError::SerializationFailed
#[test]
fn test_event_error_serialization_failed() {
    let err = EventError::SerializationFailed("bad data".to_string());
    assert!(err.to_string().contains("bad data"));
}
