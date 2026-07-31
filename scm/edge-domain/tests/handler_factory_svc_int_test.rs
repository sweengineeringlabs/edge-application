//! SAF facade smoke test — `Handler` factory (`Domain.echo_handler`) is exported
//! from the crate root.
#![cfg(all(feature = "command", feature = "handler"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::DirectCommandBusRequest;
use edge_application::DomainRuntime;
use edge_application::{Domain, Handler, HandlerContext};
use edge_application_handler::{ExecutionRequest, IdRequest, PatternRequest};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

/// @covers: Domain.echo_handler — happy path: id and pattern are set from the given strings
#[test]
fn test_echo_handler_factory_sets_id_and_pattern_happy() {
    let h = Domain.echo_handler::<TextPayload>("greeter", "/greet");
    assert_eq!(h.id(IdRequest).unwrap().id, "greeter");
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "/greet");
}

/// @covers: Domain.echo_handler — error: empty id/pattern strings are still accepted, not rejected
#[test]
fn test_echo_handler_factory_accepts_empty_strings_error() {
    let h = Domain.echo_handler::<TextPayload>("", "");
    assert_eq!(h.id(IdRequest).unwrap().id, "");
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "");
}

/// @covers: Domain.echo_handler — edge: the constructed handler actually echoes its input
#[test]
fn test_echo_handler_factory_built_handler_echoes_request_edge() {
    let h = Domain.echo_handler::<TextPayload>("echo", "/echo");
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: bus.as_ref(),
        observer: observer.as_ref(),
    };
    let result = block_on(h.execute(ExecutionRequest {
        req: TextPayload("hello".to_string()),
        ctx: &ctx,
    }))
    .unwrap();
    assert_eq!(result, TextPayload("hello".to_string()));
}
