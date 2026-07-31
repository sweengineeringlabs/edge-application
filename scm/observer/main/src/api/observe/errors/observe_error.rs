//! `ObserveError` — errors produced by the observability factory.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_observer::ObserveError` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::ObserveError;
