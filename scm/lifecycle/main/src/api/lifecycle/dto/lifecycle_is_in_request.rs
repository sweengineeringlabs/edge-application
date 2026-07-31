//! [`LifecycleIsInRequest`] — request to check the current state against a candidate.

/// Request to check whether a [`Lifecycle`](crate::api::lifecycle::traits::Lifecycle)
/// is currently in `state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LifecycleIsInRequest<S> {
    /// The candidate state to compare against.
    pub state: S,
}
