//! `TransitionPolicy` — pluggable guard for allowed state transitions.

use std::fmt::Debug;

use crate::api::lifecycle::errors::LifecycleError;
use crate::api::lifecycle::dto::{TransitionAllowedRequest, TransitionAllowedResponse};

/// Decides which state transitions are permitted for a `Lifecycle`.
///
/// Implement this trait to express domain rules such as "a task cannot move
/// from `Completed` back to `Running`" without encoding those rules in the
/// generic framework.
///
/// The state type is the associated [`State`](TransitionPolicy::State),
/// matching the convention of [`Lifecycle`](crate::api::lifecycle::traits::Lifecycle) and
/// [`Repository`](https://docs.rs/edge-domain-repository).
pub trait TransitionPolicy: Send + Sync {
    /// The concrete state type guarded by this policy.
    type State: Copy + Eq + Debug + Send + Sync;

    /// Return whether a transition from `from` to `to` is permitted.
    fn is_allowed(
        &self,
        req: TransitionAllowedRequest<Self::State>,
    ) -> Result<TransitionAllowedResponse, LifecycleError>;
}
