//! `NoopSpan` — no-op [`Span`](crate::api::Span) marker.

/// Zero-sized [`Span`](crate::api::Span) that completes without recording.
pub struct NoopSpan;
