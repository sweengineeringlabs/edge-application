//! `ObserveBootstrap` — assembles the observability triplet.

use crate::api::observe::dto::{
    BootstrapNameRequest, BootstrapNameResponse, HandlerTracerBuildRequest,
    HandlerTracerBuildResponse, LogDrainBuildRequest, LogDrainBuildResponse,
    MetricRegistryBuildRequest, MetricRegistryBuildResponse, ValidationRequest, ValidationResponse,
};
use crate::api::observe::errors::ObserveError;
use crate::api::observe::std_observe_factory::StdObserveFactory;

/// Constructs the three observability primitives as a unit.
///
/// SDK-backed factories (OTel, Prometheus) override `build_*` methods.
/// The noop `StdObserveFactory` is suitable for local dev and unit tests.
pub trait ObserveBootstrap: Send + Sync {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(
        &self,
        _req: BootstrapNameRequest,
    ) -> Result<BootstrapNameResponse, ObserveError> {
        Ok(BootstrapNameResponse { name: "observe" })
    }

    /// Return the standard (noop) observe factory instance.
    fn std_factory() -> StdObserveFactory
    where
        Self: Sized,
    {
        StdObserveFactory
    }

    /// Check whether the observability backend is reachable.
    ///
    /// Returns [`ObserveError`] when the backend is unavailable or not yet
    /// initialised. Always returns `Ok(())` for noop implementations.
    fn validate(&self, _req: ValidationRequest) -> Result<ValidationResponse, ObserveError> {
        Ok(ValidationResponse)
    }

    /// Build a [`HandlerTracer`](super::HandlerTracer).
    fn build_handler_tracer(
        &self,
        req: HandlerTracerBuildRequest,
    ) -> Result<HandlerTracerBuildResponse, ObserveError>;

    /// Build a [`MetricRegistry`](super::MetricRegistry).
    fn build_metric_registry(
        &self,
        req: MetricRegistryBuildRequest,
    ) -> Result<MetricRegistryBuildResponse, ObserveError>;

    /// Build a [`LogDrain`](super::LogDrain).
    fn build_log_drain(
        &self,
        req: LogDrainBuildRequest,
    ) -> Result<LogDrainBuildResponse, ObserveError>;
}
