//! Integration coverage for `edge-domain-event` and `edge-domain-command` dependencies.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::{Command, CommandError, ExecutionRequest, NameRequest};
use edge_application_event::{DomainEvent, EventAggregateIdRequest, EventTypeRequest};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct DepEvt;
impl DomainEvent for DepEvt {}

struct DepCmd;
impl Command for DepCmd {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[test]
fn test_domain_event_default_impls_are_callable_happy() {
    let e = DepEvt;
    assert_eq!(e.event_type(EventTypeRequest).unwrap().event_type, "event");
    assert_eq!(
        e.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        ""
    );
}

#[test]
fn test_command_execute_returns_ok_error() {
    let result = block_on(DepCmd.execute(ExecutionRequest));
    result.expect("command execute should return Ok(())");
}

#[test]
fn test_command_name_default_is_command_edge() {
    assert_eq!(DepCmd.name(NameRequest).unwrap().name, "command");
}
