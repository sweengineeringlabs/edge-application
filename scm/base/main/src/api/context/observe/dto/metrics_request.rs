//! [`MetricsRequest`] — zero-sized marker for querying the active `MetricRegistry`.

/// Request for the active [`MetricRegistry`](crate::api::context::observe::MetricRegistry).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MetricsRequest;
