//! [`TryDrainResponse`] — wrapper for the number of events drained.

/// Result of [`Projection::try_drain`](crate::api::projection::traits::Projection::try_drain).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TryDrainResponse {
    /// The number of events folded into the projection.
    pub count: usize,
}
