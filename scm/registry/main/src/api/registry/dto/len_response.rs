//! [`LenResponse`] — wrapper for registry length.

/// Result of [`Registry::len`](crate::api::Registry::len).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LenResponse {
    /// The number of registered entries.
    pub count: usize,
}
