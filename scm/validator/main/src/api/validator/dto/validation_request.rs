//! [`ValidationRequest`] — zero-sized marker for requesting configuration validation.

/// Request to validate a configuration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationRequest;
