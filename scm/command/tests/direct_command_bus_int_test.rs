//! Integration tests for `DirectCommandBus` — the zero-size in-process command bus marker.

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
        Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
    }
}

/// @covers: DirectCommandBus — is a zero-sized type
#[test]
fn test_direct_command_bus_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<DirectCommandBus>(), 0);
}

/// @covers: DirectCommandBus — dispatches failing command
#[test]
fn test_direct_command_bus_dispatch_error_command_returns_err_error() {
    let bus = DirectCommandBus;
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Err_),
    }));
    assert!(result.is_err());
}

/// @covers: DirectCommandBus — usable as a `&dyn CommandBus`
#[test]
fn test_direct_command_bus_dyn_dispatch_returns_ok_edge() {
    let bus = DirectCommandBus;
    let bus_ref: &dyn CommandBus = &bus;
    let result = block_on(bus_ref.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    assert_eq!(result, Ok(()));
}
