//! `CommandBus` impl for `LoggingCommandBus` — delegates to inner bus, logs via `tracing`.

use futures::future::BoxFuture;

use crate::api::CommandBus;
use crate::api::CommandDispatchRequest;
use crate::api::CommandError;
use crate::api::LoggingCommandBus;
use crate::api::NameRequest;

impl CommandBus for LoggingCommandBus {
    fn dispatch(&self, req: CommandDispatchRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        let name = match req.command.name(NameRequest) {
            Ok(r) => r.name,
            Err(_) => "unknown".to_string(),
        };
        let inner = self.inner.clone();
        let cmd = req.command;
        Box::pin(async move {
            let result = inner
                .dispatch(CommandDispatchRequest { command: cmd })
                .await;
            match &result {
                Ok(()) => tracing::debug!(command = %name, "command dispatched ok"),
                Err(e) => tracing::debug!(command = %name, error = %e, "command dispatch error"),
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;

    use super::*;
    use crate::api::Command;
    use crate::api::DirectCommandBus;
    use crate::api::ExecutionRequest;
    use crate::api::NameResponse;
    use crate::api::NoopCommandBus;

    struct LoggingCommandBusOk;
    impl Command for LoggingCommandBusOk {
        fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
            Ok(NameResponse {
                name: "ok-cmd".to_string(),
            })
        }
        fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    struct LoggingCommandBusErr;
    impl Command for LoggingCommandBusErr {
        fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
            Ok(NameResponse {
                name: "err-cmd".to_string(),
            })
        }
        fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_command_returns_ok() {
        let bus = LoggingCommandBus {
            inner: Arc::new(NoopCommandBus),
        };
        let result = bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(LoggingCommandBusOk),
            })
            .await;
        assert!(result.is_ok());
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_error_command_propagates_err() {
        let bus = LoggingCommandBus {
            inner: Arc::new(DirectCommandBus),
        };
        let result = bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(LoggingCommandBusErr),
            })
            .await;
        assert!(result.is_err());
    }
}
