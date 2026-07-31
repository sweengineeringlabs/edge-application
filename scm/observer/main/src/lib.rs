//! # edge-domain-observer
//!
//! **Observer contract layer** — the abstraction through which handlers observe their own behavior.
//!
//! This crate provides trait contracts for **request-scoped observability**: handlers call methods on
//! [`ObserverContext`] to emit traces, logs, and metrics without knowing or caring how that telemetry
//! is collected or where it goes.
//!
//! ## What it does
//!
//! From the **handler's perspective**, this IS the observer. Handlers see:
//! - [`HandlerTracer`] — emit distributed trace spans
//! - [`LogDrain`] — write structured log records
//! - [`MetricRegistry`] — increment counters, set gauges, record histograms
//!
//! All accessed via a single [`ObserverContext`] injected at request time.
//!
//! ## How it works
//!
//! 1. **In tests:** Use [`StdObserveFactory::noop_observer_context()`] — handlers observe, but nothing happens
//! 2. **In production:** Wire SDK-backed implementations (OpenTelemetry, Prometheus) to the same interface
//!
//! The handler code never changes. The observer backend is pluggable.
//!
//! ## Example
//!
//! ```ignore
//! // Handler receives observer at dispatch time
//! let observer = StdObserveFactory::noop_observer_context();
//!
//! // Handler observes its behavior
//! observer.tracer(TracerRequest)?.tracer
//!     .start_span(SpanStartRequest { handler_id: "my_handler".into(), operation: "execute".into() })?
//!     .span.finish(SpanFinishRequest)?;
//! observer.metrics(MetricsRequest)?.metrics
//!     .counter(CounterLookupRequest { name: "requests".into() })?
//!     .counter.increment(IncrementRequest { delta: 1 })?;
//! observer.drain(DrainRequest)?.drain
//!     .emit(LogEmitRequest { level: "INFO".into(), handler_id: "handler".into(), message: "started".into() })?;
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use api::BootstrapNameRequest;
pub use api::BootstrapNameResponse;
pub use api::CounterLookupRequest;
pub use api::CounterLookupResponse;
pub use api::DrainRequest;
pub use api::DrainResponse;
pub use api::GaugeLookupRequest;
pub use api::GaugeLookupResponse;
pub use api::GaugeSetRequest;
pub use api::GaugeSetResponse;
pub use api::HandlerTracerBuildRequest;
pub use api::HandlerTracerBuildResponse;
pub use api::HistogramLookupRequest;
pub use api::HistogramLookupResponse;
pub use api::HistogramRecordRequest;
pub use api::HistogramRecordResponse;
pub use api::IncrementRequest;
pub use api::IncrementResponse;
pub use api::LogDrainBuildRequest;
pub use api::LogDrainBuildResponse;
pub use api::LogEmitRequest;
pub use api::LogEmitResponse;
pub use api::MetricRegistryBuildRequest;
pub use api::MetricRegistryBuildResponse;
pub use api::MetricsRequest;
pub use api::MetricsResponse;
pub use api::NoopCounter;
pub use api::NoopGauge;
pub use api::NoopHandlerTracer;
pub use api::NoopHistogram;
pub use api::NoopLogDrain;
pub use api::NoopMetricRegistry;
pub use api::NoopObserverContext;
pub use api::NoopSpan;
pub use api::ObserveError;
pub use api::SpanAnnotationRequest;
pub use api::SpanAnnotationResponse;
pub use api::SpanFinishRequest;
pub use api::SpanFinishResponse;
pub use api::SpanStartRequest;
pub use api::SpanStartResponse;
pub use api::StdObserveFactory;
pub use api::TracerRequest;
pub use api::TracerResponse;
pub use api::ValidationRequest;
pub use api::ValidationResponse;
pub use saf::Counter;
pub use saf::Gauge;
pub use saf::HandlerTracer;
pub use saf::Histogram;
pub use saf::LogDrain;
pub use saf::MetricRegistry;
pub use saf::NoopObserve;
pub use saf::ObserveBootstrap;
pub use saf::ObserverContext;
pub use saf::Span;
pub use saf::COUNTER_SVC;
pub use saf::COUNTER_SVC_FACTORY;
pub use saf::GAUGE_SVC;
pub use saf::GAUGE_SVC_FACTORY;
pub use saf::HANDLER_TRACER_SVC;
pub use saf::HANDLER_TRACER_SVC_FACTORY;
pub use saf::HISTOGRAM_SVC;
pub use saf::HISTOGRAM_SVC_FACTORY;
pub use saf::LOG_DRAIN_SVC;
pub use saf::LOG_DRAIN_SVC_FACTORY;
pub use saf::METRIC_REGISTRY_SVC;
pub use saf::METRIC_REGISTRY_SVC_FACTORY;
pub use saf::NOOP_OBSERVE_SVC;
pub use saf::NOOP_OBSERVE_SVC_FACTORY;
pub use saf::OBSERVER_CONTEXT_SVC_FACTORY;
pub use saf::OBSERVE_BOOTSTRAP_SVC_FACTORY;
pub use saf::OBSERVE_CONTEXT_SVC;
pub use saf::OBSERVE_FACTORY_SVC;
pub use saf::SPAN_SVC;
pub use saf::SPAN_SVC_FACTORY;
