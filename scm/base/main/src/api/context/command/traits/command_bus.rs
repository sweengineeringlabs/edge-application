//! `CommandBus` trait — dispatches commands to their executors.

use std::future::Future;
use std::pin::Pin;

use crate::api::context::command::dto::CommandDispatchRequest;
use crate::api::context::command::errors::CommandError;

/// Dispatches [`Command`](super::command::Command) instances to their executors.
///
/// The bus decouples the caller from the command implementation.
pub trait CommandBus: Send + Sync {
    /// Dispatch a command. Returns `Err` if execution fails.
    fn dispatch(
        &self,
        req: CommandDispatchRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + '_>>;
}
