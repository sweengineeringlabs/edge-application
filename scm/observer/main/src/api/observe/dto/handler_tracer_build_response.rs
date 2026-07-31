//! [`HandlerTracerBuildResponse`] — wrapper for a constructed `HandlerTracer`.
// @allow: dto_types_must_serialize — holds a live `Box<dyn HandlerTracer>` factory
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::HandlerTracer;

/// Result of [`ObserveBootstrap::build_handler_tracer`](crate::api::ObserveBootstrap::build_handler_tracer).
pub struct HandlerTracerBuildResponse {
    /// The constructed handler tracer.
    pub tracer: Box<dyn HandlerTracer>,
}
