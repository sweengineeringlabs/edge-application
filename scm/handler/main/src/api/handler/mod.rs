//! `Handler` theme — request/response execution port contracts.

pub mod dto;
mod echo_handler;
pub mod errors;
pub mod in_process_handler_registry;
pub mod traits;
pub mod vo;

pub use dto::{
    CommandDispatchRequest, CommandExecutionRequest, CommandNameRequest, CommandNameResponse,
    CounterLookupRequest, CounterLookupResponse, DeregisterHandlerRequest,
    DeregisterHandlerResponse, DrainRequest, DrainResponse, EmptinessRequest, EmptinessResponse,
    ExecutionRequest, GaugeLookupRequest, GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse,
    HandlerLookupRequest, HandlerLookupResponse, HealthCheckRequest, HealthCheckResponse,
    HistogramLookupRequest, HistogramLookupResponse, HistogramRecordRequest,
    HistogramRecordResponse, IdRequest, IdResponse, IncrementRequest, IncrementResponse,
    LenRequest, LenResponse, ListIdsRequest, ListIdsResponse, LogEmitRequest, LogEmitResponse,
    MetricsRequest, MetricsResponse, PatternRequest, PatternResponse, RegisterHandlerRequest,
    RegisterHandlerResponse, Request, Response, SpanAnnotationRequest, SpanAnnotationResponse,
    SpanFinishRequest, SpanFinishResponse, SpanStartRequest, SpanStartResponse, TracerRequest,
    TracerResponse,
};
pub use echo_handler::EchoHandler;
pub use edge_application_base::CommandError;
pub use edge_application_base::ObserveError;
pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use traits::{
    Command, CommandBus, Counter, Gauge, Handler, HandlerRegistry, HandlerTracer, Histogram,
    LogDrain, MetricRegistry, ObserverContext, SecurityPrincipal, Span,
};
pub use vo::HandlerContext;
