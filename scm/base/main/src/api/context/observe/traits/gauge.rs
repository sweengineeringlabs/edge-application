//! `Gauge` — a point-in-time value metric.

use crate::api::context::observe::dto::{GaugeSetRequest, GaugeSetResponse};
use crate::api::context::observe::errors::ObserveError;

/// A metric that records a current absolute value (e.g. queue depth).
pub trait Gauge: Send + Sync {
    /// Set the gauge to `value`.
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, ObserveError>;
}
