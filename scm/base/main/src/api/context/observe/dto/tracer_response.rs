//! [`TracerResponse`] — wrapper for the active `HandlerTracer`.
// @allow: dto_types_must_serialize — holds a `&dyn HandlerTracer` reference, not
// wire-format data; a trait object reference cannot derive Serialize/Deserialize.

use crate::api::context::observe::HandlerTracer;

/// Result of [`ObserverContext::tracer`](crate::api::context::observe::ObserverContext::tracer).
pub struct TracerResponse<'a> {
    /// The active handler tracer.
    pub tracer: &'a dyn HandlerTracer,
}
