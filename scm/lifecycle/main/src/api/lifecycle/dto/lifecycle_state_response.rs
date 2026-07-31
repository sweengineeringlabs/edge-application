//! [`LifecycleStateResponse`] — wrapper for the current state.

/// Result of [`Lifecycle::state`](crate::api::lifecycle::traits::Lifecycle::state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LifecycleStateResponse<S> {
    /// The current state.
    pub state: S,
}
