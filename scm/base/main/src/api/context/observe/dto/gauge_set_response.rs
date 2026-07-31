//! [`GaugeSetResponse`] — wrapper for a successful gauge update.

/// Result of [`Gauge::set`](crate::api::context::observe::Gauge::set).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GaugeSetResponse;
