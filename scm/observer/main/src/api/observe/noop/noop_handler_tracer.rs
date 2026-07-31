//! `NoopHandlerTracer` — no-op [`HandlerTracer`](crate::api::HandlerTracer) marker.

/// Zero-sized [`HandlerTracer`](crate::api::HandlerTracer) that produces silent spans.
pub struct NoopHandlerTracer;
