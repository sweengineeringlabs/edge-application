//! [`LifecycleTransitionRequest`] — request to transition to a target state.

/// Request to transition a [`Lifecycle`](crate::api::lifecycle::traits::Lifecycle) to `target`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LifecycleTransitionRequest<S> {
    /// The state to transition to.
    pub target: S,
}
