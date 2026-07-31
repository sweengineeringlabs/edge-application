//! `NoopCounter` — no-op [`Counter`](crate::api::Counter) marker.

/// Zero-sized [`Counter`](crate::api::Counter) that discards every increment.
pub struct NoopCounter;
