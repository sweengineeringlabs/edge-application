//! `Command` and `CommandBus` impls for noop types — both discard/no-op execution.

use futures::future::BoxFuture;

use crate::api::Command;
use crate::api::CommandBus;
use crate::api::CommandDispatchRequest;
use crate::api::CommandError;
use crate::api::ExecutionRequest;
use crate::api::NoopCommand;
use crate::api::NoopCommandBus;

impl Command for NoopCommand {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

impl CommandBus for NoopCommandBus {
    fn dispatch(&self, _: CommandDispatchRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}
