//! [`LogEmitRequest`] — request to emit a structured log record.

/// Request to emit a structured log entry to a [`LogDrain`](crate::api::context::observe::LogDrain).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LogEmitRequest {
    /// Severity level as a string (e.g. `"INFO"`, `"WARN"`, `"ERROR"`).
    pub level: String,
    /// Handler that produced this record.
    pub handler_id: String,
    /// Log message body.
    pub message: String,
}
