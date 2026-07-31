//! [`SpanFinishResponse`] — wrapper for a successfully finished span.

/// Result of [`Span::finish`](crate::api::context::observe::Span::finish).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpanFinishResponse;
