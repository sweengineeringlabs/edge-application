//! `Counter` — a monotonically increasing metric.

use crate::api::context::observe::dto::{IncrementRequest, IncrementResponse};
use crate::api::context::observe::errors::ObserveError;

/// A monotonically increasing integer metric.
pub trait Counter: Send + Sync {
    /// Increment the counter by `delta`.
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, ObserveError>;
}
