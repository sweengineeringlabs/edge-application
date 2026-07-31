//! `ObserverContext` trait — composition handle bundling all observability primitives.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::ObserverContext` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::ObserverContext;
