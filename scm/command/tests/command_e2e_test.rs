//! SAF facade tests — `Command` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::{Command, CommandError, ExecutionRequest, NameRequest, NameResponse};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ping(String);
impl Command for Ping {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: self.0.clone(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Fails;
impl Command for Fails {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
    }
}

/// @covers: Command::name — configured name returned
#[test]
fn test_name_configured_value_returned_happy() {
    let response = Ping("create".into())
        .name(NameRequest)
        .expect("name should succeed");
    assert_eq!(response.name, "create");
}

/// @covers: Command::name — default impl returns "command"
#[test]
fn test_name_default_impl_returns_command_error() {
    let response = Fails.name(NameRequest).expect("name should succeed");
    assert_eq!(response.name, "command");
}

/// @covers: Command::name — via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_returns_name_edge() {
    let c: &dyn Command = &Ping("x".into());
    let response = c.name(NameRequest).expect("name should succeed");
    assert_eq!(response.name, "x");
}

/// @covers: Command::execute — success
#[test]
fn test_execute_ok_command_returns_ok_happy() {
    assert_eq!(
        block_on(Ping("ok".into()).execute(ExecutionRequest)),
        Ok(())
    );
}

/// @covers: Command::execute — failure propagates
#[test]
fn test_execute_failing_command_returns_err_error() {
    assert!(block_on(Fails.execute(ExecutionRequest)).is_err());
}

/// @covers: Command::execute — repeated execution is independent
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let c = Ping("x".into());
    assert_eq!(block_on(c.execute(ExecutionRequest)), Ok(()));
    assert_eq!(block_on(c.execute(ExecutionRequest)), Ok(()));
}
