//! `TransitionPolicy` impl for `PermissivePolicy`, plus its constructor.

use std::fmt::Debug;
use std::marker::PhantomData;

use crate::api::LifecycleError;
use crate::api::PermissivePolicy;
use crate::api::TransitionPolicy;
use crate::api::{TransitionAllowedRequest, TransitionAllowedResponse};

impl<S> PermissivePolicy<S> {
    /// Construct a new permissive policy.
    pub fn new() -> Self {
        PermissivePolicy(PhantomData)
    }
}

impl<S> Default for PermissivePolicy<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Copy + Eq + Debug + Send + Sync + 'static> TransitionPolicy for PermissivePolicy<S> {
    type State = S;

    fn is_allowed(
        &self,
        _req: TransitionAllowedRequest<S>,
    ) -> Result<TransitionAllowedResponse, LifecycleError> {
        Ok(TransitionAllowedResponse { allowed: true })
    }
}
