//! `Observe` theme — observability contract shape shared with `HandlerContext`.
//!
//! This is the trait/DTO *contract* only (`ObserverContext`, `HandlerTracer`, `LogDrain`,
//! `MetricRegistry`, `Counter`, `Gauge`, `Histogram`, `Span`, and their DTOs/`ObserveError`)
//! -- concrete implementations (`NoopObserve`, `StdObserveFactory`, `NoopCounter`, etc.) and
//! the bootstrap/factory traits (`NoopObserve`, `ObserveBootstrap`) that construct them stay
//! in `edge-application-observer`, which depends on this crate for the trait shape rather
//! than declaring its own.

pub mod dto;
pub mod errors;
pub mod traits;

// Every DTO referenced in a trait method signature must be flattened here, not just the
// ones ObserverContext's own methods return -- Counter/Gauge/Histogram/LogDrain/
// HandlerTracer/Span/MetricRegistry are each independently implementable by an external
// consumer, and Rust requires every type in a trait impl's signature to be nameable from
// the implementor's crate.
pub use dto::{
    CounterLookupRequest, CounterLookupResponse, DrainRequest, DrainResponse, GaugeLookupRequest,
    GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse, HistogramLookupRequest,
    HistogramLookupResponse, HistogramRecordRequest, HistogramRecordResponse, IncrementRequest,
    IncrementResponse, LogEmitRequest, LogEmitResponse, MetricsRequest, MetricsResponse,
    SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest, SpanFinishResponse,
    SpanStartRequest, SpanStartResponse, TracerRequest, TracerResponse,
};
pub use errors::ObserveError;
pub use traits::{Counter, Gauge, HandlerTracer, Histogram, LogDrain, MetricRegistry, ObserverContext, Span};
