//! Integration tests for `NoopCommandBus` — discards every command silently.

use edge_application_command::{
    Command, CommandBus, CommandDispatchRequest, CommandError, ExecutionRequest, NoopCommandBus,
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

/// @covers: NoopCommandBus — is a zero-sized type
#[test]
fn test_noop_command_bus_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NoopCommandBus>(), 0);
}

/// @covers: NoopCommandBus::dispatch — returns Ok for a successful command
#[test]
fn test_noop_command_bus_dispatch_ok_command_returns_ok_happy() {
    let bus = NoopCommandBus;
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    assert_eq!(result, Ok(()));
}

/// @covers: NoopCommandBus::dispatch — returns Ok even for a failing command
#[test]
fn test_noop_command_bus_dispatch_error_command_still_returns_ok_error() {
    let bus = NoopCommandBus;
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Err_),
    }));
    assert_eq!(result, Ok(()));
}

/// @covers: NoopCommandBus — usable via dyn CommandBus reference
#[test]
fn test_noop_command_bus_dyn_dispatch_returns_ok_edge() {
    let bus = NoopCommandBus;
    let bus_ref: &dyn CommandBus = &bus;
    let result = block_on(bus_ref.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    assert_eq!(result, Ok(()));
}
