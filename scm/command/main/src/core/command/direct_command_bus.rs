//! `CommandBus` impl for the [`DirectCommandBus`] marker — inline dispatch, no queuing.

use futures::future::BoxFuture;

use crate::api::CommandBus;
use crate::api::CommandDispatchRequest;
use crate::api::CommandError;
use crate::api::DirectCommandBus;
use crate::api::ExecutionRequest;

impl CommandBus for DirectCommandBus {
    fn dispatch(&self, req: CommandDispatchRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { req.command.execute(ExecutionRequest).await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Command;
    use crate::api::NameRequest;
    use crate::api::NameResponse;

    struct DirectCommandBusOk;
    impl Command for DirectCommandBusOk {
        fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
            Ok(NameResponse {
                name: "ok".to_string(),
            })
        }
        fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_command_returns_ok() {
        assert!(DirectCommandBus
            .dispatch(CommandDispatchRequest {
                command: Box::new(DirectCommandBusOk)
            })
            .await
            .is_ok());
    }
}
