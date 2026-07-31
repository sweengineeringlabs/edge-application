//! `Command` trait — a write operation that mutates domain state.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::Command` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::Command;
