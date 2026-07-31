//! `RepositoryError` — errors returned by `Repository` operations.

use thiserror::Error;

/// Errors that can occur during repository operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum RepositoryError {
    /// The requested entity was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// A conflicting entity already exists.
    #[error("conflict: {0}")]
    Conflict(String),
    /// The repository is temporarily unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// An internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
