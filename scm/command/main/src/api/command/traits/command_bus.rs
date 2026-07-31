//! `CommandBus` trait — dispatches commands to their executors.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_command::CommandBus` keeps resolving for existing consumers
//! while being the exact same type base-dependent crates consume directly.
//! See issue #145.

pub use edge_application_base::CommandBus;
