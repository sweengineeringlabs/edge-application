//! SAF facade tests — `CommandBus` trait via `DirectCommandBus`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::{
    Command, CommandBus, CommandDispatchRequest, CommandError, DirectCommandBus, ExecutionRequest,
};
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
        Box::pin(async { Err(CommandError::Internal("boom".into())) })
    }
}

/// @covers: CommandBus::dispatch — success
#[test]
fn test_dispatch_ok_command_returns_ok_happy() {
    let bus = DirectCommandBus;
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    let unit = result.expect("dispatch should succeed");
    assert_eq!(unit, (), "dispatch result should be unit");
}

/// @covers: CommandBus::dispatch — failure propagates
#[test]
fn test_dispatch_failing_command_returns_err_error() {
    let bus = DirectCommandBus;
    assert!(block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Err_)
    }))
    .is_err());
}

/// @covers: CommandBus::dispatch — multiple dispatches independent
#[test]
fn test_dispatch_multiple_sequential_commands_are_independent_edge() {
    let bus = DirectCommandBus;
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
