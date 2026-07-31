//! `ValidatorError` — error produced by [`Validator`](crate::Validator) checks.

use thiserror::Error;

/// Error produced when configuration validation fails.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidatorError {
    /// The configuration contained an invalid combination of fields.
    #[error("invalid configuration: {0}")]
    Invalid(String),
}
