//! # edge-domain-handler
//!
//! The `Handler` port contract — request/response execution units with registry and context.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

// Types that are part of trait signatures
pub use api::CommandDispatchRequest;
pub use api::CommandError;
pub use api::CommandExecutionRequest;
pub use api::CommandNameRequest;
pub use api::CommandNameResponse;
pub use api::CounterLookupRequest;
pub use api::CounterLookupResponse;
pub use api::DeregisterHandlerRequest;
pub use api::DeregisterHandlerResponse;
pub use api::DrainRequest;
pub use api::DrainResponse;
pub use api::EchoHandler;
pub use api::EmptinessRequest;
pub use api::EmptinessResponse;
pub use api::ExecutionRequest;
pub use api::GaugeLookupRequest;
pub use api::GaugeLookupResponse;
pub use api::GaugeSetRequest;
pub use api::GaugeSetResponse;
pub use api::HandlerContext;
pub use api::HandlerError;
pub use api::HandlerLookupRequest;
pub use api::HandlerLookupResponse;
pub use api::HealthCheckRequest;
pub use api::HealthCheckResponse;
pub use api::HistogramLookupRequest;
pub use api::HistogramLookupResponse;
pub use api::HistogramRecordRequest;
pub use api::HistogramRecordResponse;
pub use api::IdRequest;
pub use api::IdResponse;
pub use api::InProcessHandlerRegistry;
pub use api::IncrementRequest;
pub use api::IncrementResponse;
pub use api::LenRequest;
pub use api::LenResponse;
pub use api::ListIdsRequest;
pub use api::ListIdsResponse;
pub use api::LogEmitRequest;
pub use api::LogEmitResponse;
pub use api::MetricsRequest;
pub use api::MetricsResponse;
pub use api::ObserveError;
pub use api::PatternRequest;
pub use api::PatternResponse;
pub use api::RegisterHandlerRequest;
pub use api::RegisterHandlerResponse;
pub use api::Request;
pub use api::Response;
pub use api::SpanAnnotationRequest;
pub use api::SpanAnnotationResponse;
pub use api::SpanFinishRequest;
pub use api::SpanFinishResponse;
pub use api::SpanStartRequest;
pub use api::SpanStartResponse;
pub use api::TracerRequest;
pub use api::TracerResponse;

// Trait contracts
pub use api::Command;
pub use api::CommandBus;
pub use api::Counter;
pub use api::Gauge;
pub use api::Handler;
pub use api::HandlerRegistry;
pub use api::HandlerTracer;
pub use api::Histogram;
pub use api::LogDrain;
pub use api::MetricRegistry;
pub use api::ObserverContext;
pub use api::SecurityPrincipal;
pub use api::Span;

// SAF service identity constants
pub use saf::COMMAND_BUS_SVC;
pub use saf::COMMAND_BUS_SVC_FACTORY;
pub use saf::COMMAND_SVC;
pub use saf::COMMAND_SVC_FACTORY;
pub use saf::COUNTER_SVC;
pub use saf::COUNTER_SVC_FACTORY;
pub use saf::GAUGE_SVC;
pub use saf::GAUGE_SVC_FACTORY;
pub use saf::HANDLER_REGISTRY_SVC;
pub use saf::HANDLER_REGISTRY_SVC_FACTORY;
pub use saf::HANDLER_SVC;
pub use saf::HANDLER_SVC_FACTORY;
pub use saf::HANDLER_TRACER_SVC;
pub use saf::HANDLER_TRACER_SVC_FACTORY;
pub use saf::HISTOGRAM_SVC;
pub use saf::HISTOGRAM_SVC_FACTORY;
pub use saf::LOG_DRAIN_SVC;
pub use saf::LOG_DRAIN_SVC_FACTORY;
pub use saf::METRIC_REGISTRY_SVC;
pub use saf::METRIC_REGISTRY_SVC_FACTORY;
pub use saf::OBSERVER_CONTEXT_SVC;
pub use saf::OBSERVER_CONTEXT_SVC_FACTORY;
pub use saf::SECURITY_PRINCIPAL_SVC;
pub use saf::SECURITY_PRINCIPAL_SVC_FACTORY;
pub use saf::SPAN_SVC;
pub use saf::SPAN_SVC_FACTORY;
