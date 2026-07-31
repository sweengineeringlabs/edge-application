//! `MetricRegistry` trait — metric instrument factory.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::MetricRegistry` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::MetricRegistry;
