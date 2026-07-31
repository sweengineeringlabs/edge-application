//! [`SpanAnnotationResponse`] — wrapper for a successfully recorded span annotation.

/// Result of [`Span::record`](crate::api::context::observe::Span::record).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpanAnnotationResponse;
