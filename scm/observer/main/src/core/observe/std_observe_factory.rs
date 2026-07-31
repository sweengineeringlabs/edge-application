//! `impl ObserveBootstrap for StdObserveFactory`, plus its saf-facing inherent factory methods.

use std::sync::Arc;

use crate::api::HandlerTracer;
use crate::api::HandlerTracerBuildRequest;
use crate::api::HandlerTracerBuildResponse;
use crate::api::LogDrain;
use crate::api::LogDrainBuildRequest;
use crate::api::LogDrainBuildResponse;
use crate::api::MetricRegistry;
use crate::api::MetricRegistryBuildRequest;
use crate::api::MetricRegistryBuildResponse;
use crate::api::NoopHandlerTracer;
use crate::api::NoopLogDrain;
use crate::api::NoopMetricRegistry;
use crate::api::NoopObserve;
use crate::api::ObserveBootstrap;
use crate::api::ObserveError;
use crate::api::ObserverContext;
use crate::api::StdObserveFactory;

impl ObserveBootstrap for StdObserveFactory {
    fn build_handler_tracer(
        &self,
        _req: HandlerTracerBuildRequest,
    ) -> Result<HandlerTracerBuildResponse, ObserveError> {
        Ok(HandlerTracerBuildResponse {
            tracer: Box::new(NoopHandlerTracer::new()) as Box<dyn HandlerTracer>,
        })
    }

    fn build_metric_registry(
        &self,
        _req: MetricRegistryBuildRequest,
    ) -> Result<MetricRegistryBuildResponse, ObserveError> {
        Ok(MetricRegistryBuildResponse {
            metric_registry: Box::new(NoopMetricRegistry::new()) as Box<dyn MetricRegistry>,
        })
    }

    fn build_log_drain(
        &self,
        _req: LogDrainBuildRequest,
    ) -> Result<LogDrainBuildResponse, ObserveError> {
        Ok(LogDrainBuildResponse {
            log_drain: Box::new(NoopLogDrain::new()) as Box<dyn LogDrain>,
        })
    }
}

impl StdObserveFactory {
    /// Return the standard [`StdObserveFactory`] backed by noop primitives.
    ///
    /// Wire SDK-backed factories (OTel, Prometheus) at the assembler layer.
    pub fn create_factory() -> StdObserveFactory {
        StdObserveFactory
    }

    /// Return a noop [`HandlerTracer`] — suitable for unit tests and local dev.
    pub fn noop_handler_tracer() -> Box<dyn HandlerTracer> {
        <StdObserveFactory as NoopObserve>::build_noop_handler_tracer()
    }

    /// Return a noop [`LogDrain`] — suitable for unit tests and local dev.
    pub fn noop_log_drain() -> Box<dyn LogDrain> {
        <StdObserveFactory as NoopObserve>::build_noop_log_drain()
    }

    /// Return a noop [`MetricRegistry`] — suitable for unit tests and local dev.
    pub fn noop_metric_registry() -> Box<dyn MetricRegistry> {
        <StdObserveFactory as NoopObserve>::build_noop_metric_registry()
    }

    /// Return a noop [`ObserverContext`] — suitable for unit tests and local dev.
    pub fn noop_observer_context() -> Box<dyn ObserverContext> {
        <StdObserveFactory as NoopObserve>::build_noop_observer_context()
    }

    /// Return a noop [`ObserverContext`] wrapped in `Arc` — for structs that store shared ownership.
    pub fn noop_arc_observe_context() -> Arc<dyn ObserverContext> {
        Arc::from(StdObserveFactory::noop_observer_context())
    }
}
