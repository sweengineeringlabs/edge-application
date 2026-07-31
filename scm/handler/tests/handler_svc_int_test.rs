//! Integration tests — `Handler` trait via SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_application_command::{CommandBus, DirectCommandBus};
use edge_application_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, HealthCheckRequest, IdRequest,
    PatternRequest,
};
use edge_application_observer::{ObserverContext, StdObserveFactory};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct OkHandler;

#[async_trait]
impl Handler for OkHandler {
    type Request = TextPayload;
    type Response = TextPayload;

    fn id(&self, _req: IdRequest) -> Result<edge_application_handler::IdResponse, HandlerError> {
        Ok(edge_application_handler::IdResponse {
            id: "ok-handler".to_string(),
        })
    }
    fn pattern(
        &self,
        _req: PatternRequest,
    ) -> Result<edge_application_handler::PatternResponse, HandlerError> {
        Ok(edge_application_handler::PatternResponse {
            pattern: "/ok".to_string(),
        })
    }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Ok(TextPayload(req.req.0.to_uppercase()))
    }
}

struct FailHandler;

#[async_trait]
impl Handler for FailHandler {
    type Request = TextPayload;
    type Response = TextPayload;

    async fn execute(
        &self,
        _req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Err(HandlerError::ExecutionFailed("deliberate".into()))
    }
}

struct UnhealthyHandler;

#[async_trait]
impl Handler for UnhealthyHandler {
    type Request = TextPayload;
    type Response = TextPayload;

    async fn execute(
        &self,
        _req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Err(HandlerError::Unhealthy)
    }
    async fn health_check(
        &self,
        _req: HealthCheckRequest,
    ) -> Result<edge_application_handler::HealthCheckResponse, HandlerError> {
        Ok(edge_application_handler::HealthCheckResponse { healthy: false })
    }
}

fn make_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a dyn CommandBus,
    observer: &'a dyn ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: bus,
        observer,
    }
}

/// @covers: Handler::execute — success path
#[test]
fn test_execute_ok_handler_returns_response_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(OkHandler.execute(ExecutionRequest {
            req: TextPayload("hello".into()),
            ctx: &ctx
        }))
        .unwrap(),
        TextPayload("HELLO".into())
    );
}

/// @covers: Handler::execute — error propagation
#[test]
fn test_execute_failing_handler_returns_err_error() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert!(block_on(FailHandler.execute(ExecutionRequest {
        req: TextPayload("x".into()),
        ctx: &ctx
    }))
    .is_err());
}

/// @covers: Handler::id default and override
#[test]
fn test_id_returns_configured_value_happy() {
    assert_eq!(OkHandler.id(IdRequest).unwrap().id, "ok-handler");
}

/// @covers: Handler::id default
#[test]
fn test_id_default_returns_handler_edge() {
    assert_eq!(FailHandler.id(IdRequest).unwrap().id, "handler");
}

/// @covers: Handler::pattern override
#[test]
fn test_pattern_returns_configured_value_happy() {
    assert_eq!(OkHandler.pattern(PatternRequest).unwrap().pattern, "/ok");
}

/// @covers: Handler::pattern default
#[test]
fn test_pattern_default_returns_empty_edge() {
    assert_eq!(FailHandler.pattern(PatternRequest).unwrap().pattern, "");
}

/// @covers: Handler::health_check default returns true
#[test]
fn test_health_check_default_returns_true_happy() {
    assert!(
        block_on(OkHandler.health_check(HealthCheckRequest))
            .unwrap()
            .healthy
    );
}

/// @covers: Handler::health_check overridden to false
#[test]
fn test_health_check_unhealthy_handler_returns_false_error() {
    assert!(
        !block_on(UnhealthyHandler.health_check(HealthCheckRequest))
            .unwrap()
            .healthy
    );
}

/// @covers: Handler::execute — unauthenticated context threads through correctly
#[test]
fn test_execute_with_unauthenticated_context_returns_response_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(OkHandler.execute(ExecutionRequest {
            req: TextPayload("world".into()),
            ctx: &ctx
        }))
        .unwrap(),
        TextPayload("WORLD".into())
    );
}

/// @covers: Handler::execute — authenticated context threads through correctly
#[test]
fn test_execute_with_authenticated_context_still_executes_edge() {
    use edge_security_runtime::AnonymousPrincipal;
    let security = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(OkHandler.execute(ExecutionRequest {
            req: TextPayload("test".into()),
            ctx: &ctx
        }))
        .unwrap(),
        TextPayload("TEST".into())
    );
}

/// @covers: Handler::id — non-overriding impl always returns default (no error path)
#[test]
fn test_id_non_overriding_impl_returns_default_handler_error() {
    assert_eq!(FailHandler.id(IdRequest).unwrap().id, "handler");
}

/// @covers: Handler::pattern — non-overriding impl always returns empty (no error path)
#[test]
fn test_pattern_non_overriding_impl_returns_empty_string_error() {
    assert_eq!(FailHandler.pattern(PatternRequest).unwrap().pattern, "");
}
