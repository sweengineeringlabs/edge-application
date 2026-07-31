//! `Lifecycle` — generic state-machine abstraction.

use std::fmt::Debug;

use async_trait::async_trait;

use crate::api::lifecycle::errors::LifecycleError;
use crate::api::lifecycle::dto::{
    LifecycleIsInRequest, LifecycleIsInResponse, LifecycleStateRequest, LifecycleStateResponse,
    LifecycleTransitionRequest,
};

/// A state-machine abstraction for entities that transition through a typed
/// set of states.
///
/// `State` is deliberately left generic: the framework defines the substrate
/// (transition mechanics, error handling) while consumers supply the concrete
/// state type.  This keeps A2A `TaskState`, HTTP connection phases, saga
/// phases, and any other protocol-specific state space out of the framework.
///
/// The trait is `#[async_trait]`-decorated so it is object-safe and can be
/// used as `dyn Lifecycle<State = S>`.  Transitions are `async` to allow
/// implementations that emit lifecycle events, flush audit logs, or wait on
/// distributed locks before confirming a state change.
#[async_trait]
pub trait Lifecycle: Send + Sync {
    /// The concrete state type.  Must be `Copy + Eq + Debug + Send + Sync` so
    /// it can be inspected, compared, and logged without borrowing the lifecycle,
    /// and so `ManagedLifecycle<S>` satisfies the `Sync` requirement imposed by
    /// `lock_api::RwLock` on its value type.
    type State: Copy + Eq + Debug + Send + Sync;

    /// Return the current state without transitioning.
    fn state(
        &self,
        req: LifecycleStateRequest,
    ) -> Result<LifecycleStateResponse<Self::State>, LifecycleError>;

    /// Attempt to transition to the requested target.  Returns
    /// [`LifecycleError::InvalidTransition`] when the active
    /// [`TransitionPolicy`](crate::api::lifecycle::traits::TransitionPolicy)
    /// rejects the move.
    async fn transition_to(
        &self,
        req: LifecycleTransitionRequest<Self::State>,
    ) -> Result<(), LifecycleError>;

    /// Return whether the current state equals the requested state.
    fn is_in(
        &self,
        req: LifecycleIsInRequest<Self::State>,
    ) -> Result<LifecycleIsInResponse, LifecycleError> {
        let current = self.state(LifecycleStateRequest)?.state;
        Ok(LifecycleIsInResponse {
            is_in: current == req.state,
        })
    }
}
