//! Integration tests for `EchoHandler` and the `echo_handler` factory.
#![cfg(all(feature = "command", feature = "handler"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::DirectCommandBusRequest;
use edge_application::DomainRuntime;
use edge_application::{Domain, EchoHandler, Handler, HandlerContext};
use edge_application_handler::{ExecutionRequest, HealthCheckRequest, IdRequest, PatternRequest};
use edge_application_observer::{ObserverContext, StdObserveFactory};
use edge_security_runtime::SecurityContext;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NumPayload(u64);

impl edge_application_base::Request for NumPayload {}
impl edge_application_base::Response for NumPayload {}

fn make_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a dyn edge_application::CommandBus,
    observer: &'a dyn ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: bus,
        observer,
    }
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_factory_returns_arc_handler() {
    let _: Arc<dyn Handler<Request = TextPayload, Response = TextPayload>> =
        Domain.echo_handler("id", "/path");
}

/// @covers: echo_handler
#[tokio::test]
async fn test_echo_handler_returns_request_as_response() {
    let h = Domain.echo_handler::<TextPayload>("echo", "/echo");
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, bus.as_ref(), observer.as_ref());
    let result = h
        .execute(ExecutionRequest {
            req: TextPayload("hello".to_string()),
            ctx: &ctx,
        })
        .await
        .unwrap();
    assert_eq!(result, TextPayload("hello".to_string()));
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_id_matches_constructor_arg() {
    let h: Arc<dyn Handler<Request = TextPayload, Response = TextPayload>> =
        Domain.echo_handler("my-handler", "/api/v1");
    assert_eq!(h.id(IdRequest).unwrap().id, "my-handler");
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_pattern_matches_constructor_arg() {
    let h: Arc<dyn Handler<Request = TextPayload, Response = TextPayload>> =
        Domain.echo_handler("id", "/api/v1/things");
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "/api/v1/things");
}

/// @covers: EchoHandler
#[tokio::test]
async fn test_echo_handler_struct_health_check_defaults_to_true() {
    let h = EchoHandler::<TextPayload>::from(("id", "/p"));
    assert!(h.health_check(HealthCheckRequest).await.unwrap().healthy);
}

/// @covers: EchoHandler
#[tokio::test]
async fn test_echo_handler_works_with_numeric_type() {
    let h: Arc<dyn Handler<Request = NumPayload, Response = NumPayload>> =
        Domain.echo_handler("num", "/num");
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, bus.as_ref(), observer.as_ref());
    assert_eq!(
        h.execute(ExecutionRequest {
            req: NumPayload(42u64),
            ctx: &ctx
        })
        .await
        .unwrap(),
        NumPayload(42u64)
    );
}
