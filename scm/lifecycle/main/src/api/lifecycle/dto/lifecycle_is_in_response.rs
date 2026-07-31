//! [`LifecycleIsInResponse`] — wrapper for an `is_in` comparison result.

/// Result of [`Lifecycle::is_in`](crate::api::lifecycle::traits::Lifecycle::is_in).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LifecycleIsInResponse {
    /// `true` when the lifecycle's current state equals the requested state.
    pub is_in: bool,
}
