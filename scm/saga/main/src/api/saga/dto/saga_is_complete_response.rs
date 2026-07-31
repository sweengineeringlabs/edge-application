//! [`SagaIsCompleteResponse`] — wrapper for a saga completion check.

/// Result of [`Saga::is_complete`](crate::api::saga::traits::Saga::is_complete).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SagaIsCompleteResponse {
    /// `true` if the saga has reached a terminal state.
    pub complete: bool,
}
