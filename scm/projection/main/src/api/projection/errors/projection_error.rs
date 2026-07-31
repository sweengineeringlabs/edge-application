use thiserror::Error;

/// Errors that can occur when driving a projection.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProjectionError {
    /// The caller supplied an empty event slice to a batch operation.
    #[error("event stream is empty")]
    EmptyStream,

    /// An unexpected error occurred inside a projection operation.
    #[error("projection internal error: {0}")]
    Internal(String),
}
