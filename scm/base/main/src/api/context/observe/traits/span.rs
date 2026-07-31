//! `Span` — a single unit of traced work.

use crate::api::context::observe::errors::ObserveError;
use crate::api::context::observe::dto::{
    SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest, SpanFinishResponse,
};

/// A tracing span produced by [`HandlerTracer`].
///
/// [`HandlerTracer`]: super::HandlerTracer
pub trait Span: Send + Sync {
    /// Attach a key-value annotation to this span.
    fn record(&self, req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, ObserveError>;

    /// Mark this span as finished.
    fn finish(&self, req: SpanFinishRequest) -> Result<SpanFinishResponse, ObserveError>;
}
