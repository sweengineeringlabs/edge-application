//! `NoopHistogram` — no-op [`Histogram`](crate::api::Histogram) marker.

/// Zero-sized [`Histogram`](crate::api::Histogram) that discards every record.
pub struct NoopHistogram;
