//! # edge-domain-base
//!
//! The home for foundational, broadly-reusable contracts multiple otherwise-independent
//! domain ports need to agree on canonically, so no port ends up mirroring another's trait
//! behind a `no_foreign_type` bridge.
//!
//! Two cohesive sub-domains today:
//!
//! - `base` — the `Request`/`Response` marker-trait contract shared by `edge-domain-handler`
//!   and `edge-domain-service`. `Send + 'static` alone is not a contract — any type satisfies
//!   it. `Request` and `Response` give `Handler`/`Service` implementors a real, checkable
//!   bound for "valid request" and "valid response" instead of an unconstrained associated
//!   type, so a type crossing the `Service`→`Handler` bridge only ever needs to satisfy one
//!   pair of traits, not two independently-declared mirrors.
//! - `context` — the trait shapes `HandlerContext` bundles on every request:
//!   `SecurityPrincipal`, `CommandBus`/`Command`, `ObserverContext` and its family. None of
//!   `domain-handler`/`domain-command`/`domain-observer` owns these relative to the others;
//!   each depends on this crate for the trait shape and keeps its own concrete machinery.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::EmptyRequest;
pub use api::EmptyResponse;
pub use api::Request;
pub use api::RequestError;
pub use api::Response;
pub use api::ResponseError;
pub use api::ValidationRequest;
pub use api::ValidationResponse;

pub use api::Command;
pub use api::CommandBus;
pub use api::CommandDispatchRequest;
pub use api::CommandError;
pub use api::CommandExecutionRequest;
pub use api::CommandNameRequest;
pub use api::CommandNameResponse;
pub use api::Counter;
pub use api::CounterLookupRequest;
pub use api::CounterLookupResponse;
pub use api::DrainRequest;
pub use api::DrainResponse;
pub use api::Gauge;
pub use api::GaugeLookupRequest;
pub use api::GaugeLookupResponse;
pub use api::GaugeSetRequest;
pub use api::GaugeSetResponse;
pub use api::HandlerTracer;
pub use api::Histogram;
pub use api::HistogramLookupRequest;
pub use api::HistogramLookupResponse;
pub use api::HistogramRecordRequest;
pub use api::HistogramRecordResponse;
pub use api::IncrementRequest;
pub use api::IncrementResponse;
pub use api::LogDrain;
pub use api::LogEmitRequest;
pub use api::LogEmitResponse;
pub use api::MetricRegistry;
pub use api::MetricsRequest;
pub use api::MetricsResponse;
pub use api::ObserveError;
pub use api::ObserverContext;
pub use api::SecurityPrincipal;
pub use api::Span;
pub use api::SpanAnnotationRequest;
pub use api::SpanAnnotationResponse;
pub use api::SpanFinishRequest;
pub use api::SpanFinishResponse;
pub use api::SpanStartRequest;
pub use api::SpanStartResponse;
pub use api::TracerRequest;
pub use api::TracerResponse;

pub use saf::REQUEST_SVC;
pub use saf::REQUEST_SVC_FACTORY;
pub use saf::RESPONSE_SVC;
pub use saf::RESPONSE_SVC_FACTORY;

pub use saf::COMMAND_BUS_SVC;
pub use saf::COMMAND_BUS_SVC_FACTORY;
pub use saf::COMMAND_SVC;
pub use saf::COMMAND_SVC_FACTORY;
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
pub use saf::OBSERVER_CONTEXT_SVC;
pub use saf::OBSERVER_CONTEXT_SVC_FACTORY;
pub use saf::SECURITY_PRINCIPAL_SVC;
pub use saf::SECURITY_PRINCIPAL_SVC_FACTORY;
pub use saf::SPAN_SVC;
pub use saf::SPAN_SVC_FACTORY;
