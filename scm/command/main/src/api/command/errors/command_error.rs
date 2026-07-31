//! Error type for [`Command`](super::super::traits::Command) operations.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_command::CommandError` keeps resolving for existing consumers.
//! See issue #145.

pub use edge_application_base::CommandError;
