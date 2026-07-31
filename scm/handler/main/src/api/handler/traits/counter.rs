//! `Counter` trait — a monotonically increasing metric.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::Counter` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::Counter;
