//! Integration tests for `LoggingCommandBus` — wraps inner bus and logs via tracing.

use std::sync::Arc;

use edge_application_command::{
    Command, CommandBus, CommandDispatchRequest, CommandError, DirectCommandBus, ExecutionRequest,
    LoggingCommandBus, NameRequest, NameResponse, NoopCommandBus,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_;
impl Command for Ok_ {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "ok".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Err_;
impl Command for Err_ {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "err".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
    }
}

fn noop_inner() -> Arc<dyn CommandBus> {
    Arc::new(NoopCommandBus)
}

fn direct_inner() -> Arc<dyn CommandBus> {
    Arc::new(DirectCommandBus)
}

/// @covers: LoggingCommandBus — constructs directly from inner field
#[test]
fn test_logging_command_bus_new_from_noop_inner_happy() {
    let bus = LoggingCommandBus {
        inner: noop_inner(),
    };
    let _: &dyn CommandBus = &bus;
}

/// @covers: LoggingCommandBus::dispatch — propagates Ok from inner bus
#[test]
fn test_logging_command_bus_dispatch_ok_command_returns_ok_happy() {
    let bus = LoggingCommandBus { inner: noop_inner() };
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    assert_eq!(result, Ok(()));
}

/// @covers: LoggingCommandBus::dispatch — propagates Err from inner bus
#[test]
fn test_logging_command_bus_dispatch_error_command_returns_err_error() {
    let bus = LoggingCommandBus { inner: direct_inner() };
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Err_),
    }));
    assert!(result.is_err());
}

/// @covers: LoggingCommandBus — usable via dyn CommandBus reference
#[test]
fn test_logging_command_bus_dyn_dispatch_edge() {
    let bus = LoggingCommandBus { inner: noop_inner() };
    let bus_ref: &dyn CommandBus = &bus;
    let result = block_on(bus_ref.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    assert_eq!(result, Ok(()));
}
