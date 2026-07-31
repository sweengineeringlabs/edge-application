//! SAF facade tests — `CommandBus` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_base::{Command, CommandBus, CommandDispatchRequest, CommandError, CommandExecutionRequest};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_;
impl Command for Ok_ {
    fn execute(&self, _req: CommandExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Err_;
impl Command for Err_ {
    fn execute(&self, _req: CommandExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::Internal("boom".into())) })
    }
}

/// Directly runs the dispatched command's `execute`, mirroring the simplest
/// legal `CommandBus` implementation a consumer could provide.
struct TestBus;
impl CommandBus for TestBus {
    fn dispatch(
        &self,
        req: CommandDispatchRequest,
    ) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { req.command.execute(CommandExecutionRequest).await })
    }
}

/// @covers: CommandBus::dispatch — success
#[test]
fn test_dispatch_ok_command_returns_ok_happy() {
    let bus = TestBus;
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    result.expect("dispatch should succeed");
}

/// @covers: CommandBus::dispatch — failure propagates
#[test]
fn test_dispatch_failing_command_returns_err_error() {
    let bus = TestBus;
    assert!(block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Err_)
    }))
    .is_err());
}

/// @covers: CommandBus::dispatch — multiple dispatches independent
#[test]
fn test_dispatch_multiple_sequential_commands_are_independent_edge() {
    let bus = TestBus;
    assert!(block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_)
    }))
    .is_ok());
    assert!(block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Err_)
    }))
    .is_err());
    assert!(block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_)
    }))
    .is_ok());
}
