//! [`CommandDispatchRequest`] — input for [`CommandBus::dispatch`](super::super::traits::CommandBus::dispatch).
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_command::CommandDispatchRequest` keeps resolving for existing
//! consumers. See issue #145.

pub use edge_application_base::CommandDispatchRequest;
