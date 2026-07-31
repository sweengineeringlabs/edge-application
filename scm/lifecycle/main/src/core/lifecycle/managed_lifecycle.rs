//! `Lifecycle` impl for `ManagedLifecycle`, plus its constructor.

use std::fmt::Debug;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::api::LifecycleError;
use crate::api::ManagedLifecycle;
use crate::api::Lifecycle;
use crate::api::TransitionPolicy;
use crate::api::{
    LifecycleStateRequest, LifecycleStateResponse, LifecycleTransitionRequest,
    TransitionAllowedRequest,
};

impl<S: Copy + Eq + Debug + Send + Sync + 'static> ManagedLifecycle<S> {
    /// Construct a new lifecycle in `initial` state, governed by `policy`.
    pub fn new(initial: S, policy: Box<dyn TransitionPolicy<State = S>>) -> Self {
        Self {
            state: RwLock::new(initial),
            policy,
        }
    }
}

#[async_trait]
impl<S: Copy + Eq + Debug + Send + Sync + 'static> Lifecycle for ManagedLifecycle<S> {
    type State = S;

    fn state(
        &self,
        _req: LifecycleStateRequest,
    ) -> Result<LifecycleStateResponse<S>, LifecycleError> {
        let state = *self.state.read().unwrap_or_else(|p| p.into_inner());
        Ok(LifecycleStateResponse { state })
    }

    async fn transition_to(
        &self,
        req: LifecycleTransitionRequest<S>,
    ) -> Result<(), LifecycleError> {
        let target = req.target;
        let from = *self.state.read().unwrap_or_else(|p| p.into_inner());
        let allowed = self
            .policy
            .is_allowed(TransitionAllowedRequest { from, to: target })?
            .allowed;
        if !allowed {
            return Err(LifecycleError::InvalidTransition {
                from: format!("{from:?}"),
                to: format!("{target:?}"),
            });
        }
        *self.state.write().unwrap_or_else(|p| p.into_inner()) = target;
        Ok(())
    }
}
