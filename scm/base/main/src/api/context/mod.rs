//! `Context` — the trait shapes `HandlerContext` bundles on every request:
//! `SecurityPrincipal`, `CommandBus`/`Command`, `ObserverContext` and its family.
//!
//! Declared here, not in `domain-handler`, `domain-command`, or `domain-observer`, because
//! none of the three owns the others -- `HandlerContext` mandates all three together, every
//! request, while `Command`/`Observer` also have real consumers independent of `Handler`
//! (e.g. a `Service` dispatching its own commands with no `Handler` involved at all). Each
//! port's own package keeps its concrete machinery and depends on this crate for the trait
//! shape, rather than one port mirroring another's contract behind a `no_foreign_type`
//! bridge. See issue #145/#146 for the full rationale.

pub mod command;
pub mod observe;
pub mod security;

pub use command::{Command, CommandBus, CommandDispatchRequest, CommandError, ExecutionRequest as CommandExecutionRequest, NameRequest as CommandNameRequest, NameResponse as CommandNameResponse};
pub use observe::{
    Counter, CounterLookupRequest, CounterLookupResponse, DrainRequest, DrainResponse, Gauge,
    GaugeLookupRequest, GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse, HandlerTracer,
    Histogram, HistogramLookupRequest, HistogramLookupResponse, HistogramRecordRequest,
    HistogramRecordResponse, IncrementRequest, IncrementResponse, LogDrain, LogEmitRequest,
    LogEmitResponse, MetricRegistry, MetricsRequest, MetricsResponse, ObserveError,
    ObserverContext, Span, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse, SpanStartRequest, SpanStartResponse, TracerRequest, TracerResponse,
};
pub use security::SecurityPrincipal;
