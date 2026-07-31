#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::{Command, ExecutionRequest, NameRequest, NoopCommand};

#[tokio::test]
async fn test_noop_command_execute_returns_ok_happy() {
    assert!(NoopCommand.execute(ExecutionRequest).await.is_ok());
}

#[test]
fn test_noop_command_name_returns_default_happy() {
    let response = NoopCommand.name(NameRequest).expect("name should succeed");
    assert_eq!(response.name, "command");
}

#[test]
fn test_noop_command_default_name_does_not_signal_failure_error() {
    // NoopCommand never signals domain failure — name must not be empty
    let response = NoopCommand.name(NameRequest).expect("name should succeed");
    assert!(!response.name.is_empty());
}

#[test]
fn test_noop_command_is_copy_type_edge() {
    let a = NoopCommand;
    let b = a;
    // Both a and b should still be valid, proving Copy semantics
    assert_eq!(
        a.name(NameRequest).expect("name should succeed"),
        b.name(NameRequest).expect("name should succeed")
    );
}
