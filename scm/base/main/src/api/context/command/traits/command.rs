//! `Command` trait — a write operation that mutates domain state.

use std::future::Future;
use std::pin::Pin;

use crate::api::context::command::dto::{ExecutionRequest, NameRequest, NameResponse};
use crate::api::context::command::errors::CommandError;

/// A named write operation that mutates domain state and returns no value.
///
/// Commands are the write side of the CQRS split. They express intent
/// ("create order", "cancel subscription") and return only success or failure.
pub trait Command: Send + Sync {
    /// Stable name identifying this command type.
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "command".to_string(),
        })
    }

    /// Execute the command, mutating domain state.
    fn execute(&self, _req: ExecutionRequest) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + '_>>;
}
