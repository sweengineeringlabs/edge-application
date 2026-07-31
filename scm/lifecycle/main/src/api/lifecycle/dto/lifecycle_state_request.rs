//! [`LifecycleStateRequest`] — zero-sized marker for querying the current state.

/// Request for a [`Lifecycle`](crate::api::lifecycle::traits::Lifecycle)'s current state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LifecycleStateRequest;
