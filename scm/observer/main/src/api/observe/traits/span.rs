//! `Span` — a single unit of traced work.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_observer::Span` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::Span;
