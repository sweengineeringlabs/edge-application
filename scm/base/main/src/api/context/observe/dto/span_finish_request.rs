//! [`SpanFinishRequest`] — zero-sized marker for marking a span finished.

/// Request to mark a [`Span`](crate::api::context::observe::Span) as finished.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpanFinishRequest;
