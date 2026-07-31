//! [`HistogramRecordResponse`] — wrapper for a successful histogram observation.

/// Result of [`Histogram::record`](crate::api::context::observe::Histogram::record).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HistogramRecordResponse;
