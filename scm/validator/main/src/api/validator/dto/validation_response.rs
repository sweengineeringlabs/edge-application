//! [`ValidationResponse`] — wrapper for successful configuration validation.

/// Result of [`Validator::validate`](crate::api::Validator::validate).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationResponse;
