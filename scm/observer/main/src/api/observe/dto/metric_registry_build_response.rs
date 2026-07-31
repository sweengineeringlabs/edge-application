//! [`MetricRegistryBuildResponse`] — wrapper for a constructed `MetricRegistry`.
// @allow: dto_types_must_serialize — holds a live `Box<dyn MetricRegistry>` factory
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::MetricRegistry;

/// Result of [`ObserveBootstrap::build_metric_registry`](crate::api::ObserveBootstrap::build_metric_registry).
pub struct MetricRegistryBuildResponse {
    /// The constructed metric registry.
    pub metric_registry: Box<dyn MetricRegistry>,
}
