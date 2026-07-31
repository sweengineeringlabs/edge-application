//! [`GaugeLookupResponse`] — wrapper for a resolved gauge instrument.
// @allow: dto_types_must_serialize — holds a live `Box<dyn Gauge>` instrument
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::context::observe::Gauge;

/// Result of [`MetricRegistry::gauge`](crate::api::context::observe::MetricRegistry::gauge).
pub struct GaugeLookupResponse {
    /// The resolved gauge instrument.
    pub gauge: Box<dyn Gauge>,
}
