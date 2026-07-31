//! `NoopMetricRegistry` — no-op [`MetricRegistry`](crate::api::MetricRegistry) marker.

/// Zero-sized [`MetricRegistry`](crate::api::MetricRegistry) that discards all instruments.
pub struct NoopMetricRegistry;
