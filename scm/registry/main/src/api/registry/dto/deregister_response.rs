//! [`DeregisterResponse`] — wrapper for deregister result.

/// Result of [`Registry::deregister`](crate::api::Registry::deregister).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeregisterResponse {
    /// `true` if an entry was present (and removed) under the requested id.
    pub was_present: bool,
}
