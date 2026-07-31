//! [`SagaIsCompleteRequest`] — zero-sized marker for querying saga completion.

/// Request to check whether a [`Saga`](crate::api::saga::traits::Saga) has reached a terminal state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SagaIsCompleteRequest;
