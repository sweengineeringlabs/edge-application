//! `CommandBus` trait — dispatches commands to their executors.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::CommandBus` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::CommandBus;
