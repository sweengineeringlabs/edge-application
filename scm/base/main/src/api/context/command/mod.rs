//! `Command` theme — CQRS write-side contract shape shared with `HandlerContext`.
//!
//! This is the trait/DTO *contract* only (`Command`, `CommandBus`,
//! `CommandDispatchRequest`, `CommandError`, `ExecutionRequest`, `NameRequest`,
//! `NameResponse`) -- concrete implementations (`DirectCommandBus`, `NoopCommand`,
//! `LoggingCommandBus`, etc.) stay in `edge-application-command`, which depends on this
//! crate for the trait shape rather than declaring its own.

pub mod dto;
pub mod errors;
pub mod traits;

pub use dto::{CommandDispatchRequest, ExecutionRequest, NameRequest, NameResponse};
pub use errors::CommandError;
pub use traits::{Command, CommandBus};
