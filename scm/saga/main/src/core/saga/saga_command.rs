//! Blanket bridge: every `edge_application_command::Command` satisfies `SagaCommand`.

use std::future::Future;
use std::pin::Pin;

use edge_application_command::{Command, ExecutionRequest};

use crate::api::SagaCommand;
use crate::api::SagaCommandDispatchRequest;
use crate::api::SagaError;

impl<T: Command> SagaCommand for T {
    fn dispatch(
        &self,
        _req: SagaCommandDispatchRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), SagaError>> + Send + '_>> {
        Box::pin(async move {
            self.execute(ExecutionRequest)
                .await
                .map_err(|e| SagaError::CommandDispatchFailed(e.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {
    use edge_application_command::CommandError;
    use futures::executor::block_on;

    use super::*;

    struct SagaCommandBridgeOkTestCmd;
    impl Command for SagaCommandBridgeOkTestCmd {
        fn execute(
            &self,
            _req: ExecutionRequest,
        ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + '_>> {
            Box::pin(async { Ok(()) })
        }
    }

    struct SagaCommandBridgeFailingTestCmd;
    impl Command for SagaCommandBridgeFailingTestCmd {
        fn execute(
            &self,
            _req: ExecutionRequest,
        ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + '_>> {
            Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
        }
    }

    #[test]
    fn test_dispatch_ok_command_returns_ok_happy() {
        assert_eq!(
            block_on(SagaCommandBridgeOkTestCmd.dispatch(SagaCommandDispatchRequest)),
            Ok(())
        );
    }

    #[test]
    fn test_dispatch_failing_command_returns_dispatch_failed_error() {
        assert_eq!(
            block_on(SagaCommandBridgeFailingTestCmd.dispatch(SagaCommandDispatchRequest)),
            Err(SagaError::CommandDispatchFailed(
                "rule violation: denied".to_string()
            ))
        );
    }

    #[test]
    fn test_dispatch_repeated_calls_are_independent_edge() {
        let c = SagaCommandBridgeOkTestCmd;
        assert_eq!(block_on(c.dispatch(SagaCommandDispatchRequest)), Ok(()));
        assert_eq!(block_on(c.dispatch(SagaCommandDispatchRequest)), Ok(()));
    }
}
