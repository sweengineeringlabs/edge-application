//! `HandlerTracer` — tracing contract for domain handlers.

use crate::api::context::observe::errors::ObserveError;
use crate::api::context::observe::dto::{SpanStartRequest, SpanStartResponse};

/// Opens tracing spans for a domain handler execution.
///
/// Spans produced here are children of the `pipeline.stage` span created by
/// `edge-dispatch` for every handler invocation.
pub trait HandlerTracer: Send + Sync {
    /// Start a new span named `operation` for handler `handler_id`.
    fn start_span(&self, req: SpanStartRequest) -> Result<SpanStartResponse, ObserveError>;
}
