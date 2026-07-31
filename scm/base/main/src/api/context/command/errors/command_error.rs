//! Error type for [`Command`](super::super::traits::Command) operations.

/// Error produced by command execution.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CommandError {
    /// The command input was invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// A business rule prevented execution.
    #[error("rule violation: {0}")]
    RuleViolation(String),
    /// The target resource was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
