//! [`HandlerTracerBuildRequest`] — zero-sized marker for requesting a `HandlerTracer`.

/// Request to build a [`HandlerTracer`](crate::api::HandlerTracer).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HandlerTracerBuildRequest;
