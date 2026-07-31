//! [`SpanStartRequest`] — request to open a new tracing span.

/// Request to start a new span named `operation` for handler `handler_id`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpanStartRequest {
    /// The handler that owns this span.
    pub handler_id: String,
    /// The operation name for the span.
    pub operation: String,
}
