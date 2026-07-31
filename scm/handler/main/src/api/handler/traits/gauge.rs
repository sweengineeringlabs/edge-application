//! `Gauge` trait — a point-in-time value metric.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::Gauge` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::Gauge;
