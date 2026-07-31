//! [`TracerRequest`] — zero-sized marker for querying the active `HandlerTracer`.

/// Request for the active [`HandlerTracer`](crate::api::context::observe::HandlerTracer).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TracerRequest;
