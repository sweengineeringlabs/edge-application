//! Configuration validation contract.

use crate::api::validator::errors::ValidatorError;
use crate::api::validator::dto::{ValidationRequest, ValidationResponse};

/// Configuration validation contract.
///
/// Implemented by configuration types to validate their fields before use.
/// Checks structural correctness (non-empty fields, range bounds) rather than
/// business intent.
pub trait Validator {
    /// Validate the configuration.
    ///
    /// Returns [`ValidatorError::Invalid`] with a human-readable description when
    /// the configuration contains an invalid combination of fields.
    fn validate(&self, req: ValidationRequest) -> Result<ValidationResponse, ValidatorError>;
}
