//! `HandlerTracer` trait — tracing contract for domain handlers.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::HandlerTracer` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::HandlerTracer;
