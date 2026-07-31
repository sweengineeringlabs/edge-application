//! `NoopGauge` — no-op [`Gauge`](crate::api::Gauge) marker.

/// Zero-sized [`Gauge`](crate::api::Gauge) that discards every set.
pub struct NoopGauge;
