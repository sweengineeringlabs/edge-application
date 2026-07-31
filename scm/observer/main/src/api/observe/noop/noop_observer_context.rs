//! `NoopObserverContext` — no-op [`ObserverContext`](crate::api::ObserverContext) marker.

use crate::api::observe::traits::HandlerTracer;
use crate::api::observe::traits::LogDrain;
use crate::api::observe::traits::MetricRegistry;

/// Bundles no-op tracer, drain, and metrics into an [`ObserverContext`](crate::api::ObserverContext).
pub struct NoopObserverContext {
    pub(crate) tracer: Box<dyn HandlerTracer>,
    pub(crate) drain: Box<dyn LogDrain>,
    pub(crate) metrics: Box<dyn MetricRegistry>,
}
