//! `Command` trait — a write operation that mutates domain state.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_command::Command` keeps resolving for existing consumers
//! while being the exact same type base-dependent crates (e.g. `edge-application-handler`)
//! consume directly. See issue #145.

pub use edge_application_base::Command;
