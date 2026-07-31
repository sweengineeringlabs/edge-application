//! Integration tests for [`NoopSagaCommand`].
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::{ExecutionRequest, NameRequest};
use edge_application_saga::{Command, NoopSagaCommand};
use futures::executor::block_on;

/// @covers: execute
#[test]
fn test_execute_noop_saga_command_returns_ok_happy() {
    let cmd = NoopSagaCommand;
    let result = block_on(cmd.execute(ExecutionRequest));
    result.expect("noop saga command execute should always succeed");
}

/// @covers: name
#[test]
fn test_name_noop_saga_command_returns_default_error() {
    // Verifies the Command default impl is inherited, not accidentally overridden
    let cmd = NoopSagaCommand;
    assert_eq!(cmd.name(NameRequest).unwrap().name, "command");
}

/// @covers: execute
#[test]
fn test_execute_noop_saga_command_can_be_called_multiple_times_edge() {
    let cmd = NoopSagaCommand;
    let r1 = block_on(cmd.execute(ExecutionRequest));
    let r2 = block_on(cmd.execute(ExecutionRequest));
    r1.expect("first execute should succeed");
    r2.expect("second execute should also succeed");
}
