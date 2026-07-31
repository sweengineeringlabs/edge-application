//! [`SpanStartResponse`] — wrapper for a newly opened tracing span.
// @allow: dto_types_must_serialize — holds a live `Box<dyn Span>` instrument
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::context::observe::Span;

/// Result of [`HandlerTracer::start_span`](crate::api::context::observe::HandlerTracer::start_span).
pub struct SpanStartResponse {
    /// The newly opened span.
    pub span: Box<dyn Span>,
}
