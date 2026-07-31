//! [`MetricRegistryBuildRequest`] — zero-sized marker for requesting a `MetricRegistry`.

/// Request to build a [`MetricRegistry`](crate::api::MetricRegistry).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MetricRegistryBuildRequest;
