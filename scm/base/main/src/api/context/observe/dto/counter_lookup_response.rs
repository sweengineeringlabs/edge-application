//! [`CounterLookupResponse`] — wrapper for a resolved counter instrument.
// @allow: dto_types_must_serialize — holds a live `Box<dyn Counter>` instrument
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::context::observe::Counter;

/// Result of [`MetricRegistry::counter`](crate::api::context::observe::MetricRegistry::counter).
pub struct CounterLookupResponse {
    /// The resolved counter instrument.
    pub counter: Box<dyn Counter>,
}
