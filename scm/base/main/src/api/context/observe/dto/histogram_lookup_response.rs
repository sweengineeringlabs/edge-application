//! [`HistogramLookupResponse`] — wrapper for a resolved histogram instrument.
// @allow: dto_types_must_serialize — holds a live `Box<dyn Histogram>` instrument
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::context::observe::Histogram;

/// Result of [`MetricRegistry::histogram`](crate::api::context::observe::MetricRegistry::histogram).
pub struct HistogramLookupResponse {
    /// The resolved histogram instrument.
    pub histogram: Box<dyn Histogram>,
}
