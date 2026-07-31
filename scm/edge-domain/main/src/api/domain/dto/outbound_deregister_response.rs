//! [`OutboundDeregisterResponse`] — wrapper for a handle removal result.

/// Result of [`OutboundRegistry::deregister`](crate::api::domain::traits::OutboundRegistry::deregister).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OutboundDeregisterResponse {
    /// `true` if a handle was present and removed.
    pub removed: bool,
}
