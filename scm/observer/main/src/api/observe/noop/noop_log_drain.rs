//! `NoopLogDrain` — no-op [`LogDrain`](crate::api::LogDrain) marker.

/// Zero-sized [`LogDrain`](crate::api::LogDrain) that discards every record.
pub struct NoopLogDrain;
