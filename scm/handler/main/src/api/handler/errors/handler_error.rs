//! Error type for [`Handler`](super::super::traits::Handler) operations.

/// Error produced by handler execution.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum HandlerError {
    /// The requested operation is not supported.
    #[error("unsupported operation: {0}")]
    Unsupported(String),
    /// The request input was invalid.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    /// The target resource was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// A conflicting state was detected.
    #[error("conflict: {0}")]
    Conflict(String),
    /// Handler execution failed with an internal error.
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
    /// The handler is in an unhealthy state and cannot process requests.
    #[error("handler unhealthy")]
    Unhealthy,
    /// A required precondition was not met.
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),
    /// The caller is not authenticated.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// The caller lacks permission for the requested operation.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    /// The operation timed out.
    #[error("timeout: {0}")]
    Timeout(String),
    /// The handler was intentionally skipped.
    #[error("handler skipped")]
    Skipped,
}
