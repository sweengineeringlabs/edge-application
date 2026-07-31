//! [`SpanAnnotationRequest`] — request to attach a key-value annotation to a span.

/// Request to attach a key-value annotation to a [`Span`](crate::api::context::observe::Span).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpanAnnotationRequest {
    /// The annotation key.
    pub key: String,
    /// The annotation value.
    pub value: String,
}
