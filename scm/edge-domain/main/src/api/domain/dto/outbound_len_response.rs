//! [`OutboundLenResponse`] — wrapper for the registry's handle count.

/// Result of [`OutboundRegistry::len`](crate::api::domain::traits::OutboundRegistry::len).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OutboundLenResponse {
    /// The number of currently registered handles.
    pub count: usize,
}
