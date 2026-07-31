//! Structural coverage for [`CommandDispatchRequest`].

use edge_application_command::{Command, CommandDispatchRequest, CommandError, ExecutionRequest};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_;
impl Command for Ok_ {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Err_;
impl Command for Err_ {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
    }
}

/// @covers: CommandDispatchRequest
#[test]
fn test_command_dispatch_request_wraps_the_given_command_happy() {
    let req = CommandDispatchRequest { command: Box::new(Ok_) };
    assert_eq!(block_on(req.command.execute(ExecutionRequest)), Ok(()));
}

/// @covers: CommandDispatchRequest
#[test]
fn test_command_dispatch_request_preserves_a_failing_commands_error_error() {
    let req = CommandDispatchRequest { command: Box::new(Err_) };
    assert!(block_on(req.command.execute(ExecutionRequest)).is_err());
}

/// @covers: CommandDispatchRequest
#[test]
fn test_command_dispatch_request_field_is_publicly_constructible_edge() {
    let req = CommandDispatchRequest { command: Box::new(Ok_) };
    assert_eq!(block_on(req.command.execute(ExecutionRequest)), Ok(()));
}
