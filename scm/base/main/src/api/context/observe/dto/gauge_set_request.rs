//! [`GaugeSetRequest`] — request to set a gauge's current value.

/// Request to set a [`Gauge`](crate::api::context::observe::Gauge) to `value`.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GaugeSetRequest {
    /// The value to record.
    pub value: f64,
}
