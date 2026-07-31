//! [`OutboundIsEmptyResponse`] — wrapper for an empty-check result.

/// Result of [`OutboundRegistry::is_empty`](crate::api::domain::traits::OutboundRegistry::is_empty).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OutboundIsEmptyResponse {
    /// `true` if the registry holds no handles.
    pub empty: bool,
}
