//! [`OutboundNamesResponse`] — wrapper for the set of registered handle names.

/// Result of [`OutboundRegistry::names`](crate::api::domain::traits::OutboundRegistry::names).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OutboundNamesResponse {
    /// Snapshot of registered names. Order is unspecified.
    pub names: Vec<String>,
}
