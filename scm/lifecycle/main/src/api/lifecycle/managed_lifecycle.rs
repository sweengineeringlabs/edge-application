//! `ManagedLifecycle` — in-process reference implementation of `Lifecycle`.

use std::fmt::Debug;
use std::sync::RwLock;

use crate::api::lifecycle::traits::TransitionPolicy;

/// An in-process [`Lifecycle`](crate::api::lifecycle::traits::Lifecycle)
/// implementation driven by a pluggable [`TransitionPolicy`].
///
/// The current state is held in a [`std::sync::RwLock`] so concurrent reads
/// are cheap and transitions are serialised without blocking an async executor.
pub struct ManagedLifecycle<S: Copy + Eq + Debug + Send + Sync + 'static> {
    pub(crate) state: RwLock<S>,
    pub(crate) policy: Box<dyn TransitionPolicy<State = S>>,
}
