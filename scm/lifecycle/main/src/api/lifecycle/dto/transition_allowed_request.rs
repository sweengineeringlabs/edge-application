//! [`TransitionAllowedRequest`] — request to check whether a transition is permitted.

/// Request to check whether a transition from `from` to `to` is permitted by a
/// [`TransitionPolicy`](crate::api::lifecycle::traits::TransitionPolicy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TransitionAllowedRequest<S> {
    /// The state the lifecycle is currently in.
    pub from: S,
    /// The state the lifecycle would transition to.
    pub to: S,
}
