//! [`HistogramLookupRequest`] — request to resolve a named histogram instrument.

/// Request for the [`Histogram`](crate::api::context::observe::Histogram) instrument named `name`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HistogramLookupRequest {
    /// The instrument name.
    pub name: String,
}
