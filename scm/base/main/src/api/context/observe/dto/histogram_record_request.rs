//! [`HistogramRecordRequest`] — request to record a histogram observation.

/// Request to record a single observation on a [`Histogram`](crate::api::context::observe::Histogram).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HistogramRecordRequest {
    /// The observed value.
    pub value: f64,
}
