//! [`ValidationResponse`] — wrapper for successful entity validation.

/// Result of [`Entity::validate`](crate::api::Entity::validate).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationResponse;
