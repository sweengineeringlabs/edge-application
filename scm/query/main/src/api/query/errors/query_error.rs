//! Error type for [`Query`](crate::Query) operations.

use thiserror::Error;

/// Error produced by query execution.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum QueryError {
    /// The query input was invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// The requested resource does not exist.
    #[error("not found: {0}")]
    NotFound(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
