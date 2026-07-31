//! # edge-domain-command
//!
//! The `Command` port contract — CQRS write-side with CommandBus and DirectCommandBus.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::CommandDispatchRequest;
pub use api::CommandError;
pub use api::DirectCommandBus;
pub use api::ExecutionRequest;
pub use api::LoggingCommandBus;
pub use api::NameRequest;
pub use api::NameResponse;
pub use api::NoopCommand;
pub use api::NoopCommandBus;
pub use saf::Command;
pub use saf::CommandBus;
pub use saf::{COMMAND_BUS_SVC_FACTORY, COMMAND_SVC_FACTORY};
