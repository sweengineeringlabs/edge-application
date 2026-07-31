use edge_application_command::{Command, CommandError, ExecutionRequest};
use futures::future::BoxFuture;

use crate::api::NoopSagaCommand;

impl Command for NoopSagaCommand {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}
