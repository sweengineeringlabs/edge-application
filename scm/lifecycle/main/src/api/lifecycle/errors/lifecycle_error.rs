//! `LifecycleError` — error variants for lifecycle state transitions.

/// Errors returned by [`Lifecycle::transition_to`](crate::api::lifecycle::traits::Lifecycle::transition_to).
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum LifecycleError {
    /// The active [`TransitionPolicy`](crate::api::lifecycle::traits::TransitionPolicy)
    /// rejected the transition from `from` to `to`.
    #[error("transition from `{from}` to `{to}` is not allowed")]
    InvalidTransition {
        /// Debug representation of the state the lifecycle was in.
        from: String,
        /// Debug representation of the state the lifecycle was asked to enter.
        to: String,
    },
}
