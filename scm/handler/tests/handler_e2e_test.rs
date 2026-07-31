//! End-to-end contract tests for the `Handler` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, HealthCheckRequest,
    HealthCheckResponse, IdRequest, PatternRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct EchoHandlerDouble;

#[async_trait::async_trait]
impl Handler for EchoHandlerDouble {
    type Request = TextPayload;
    type Response = TextPayload;

    async fn execute(
        &self,
        req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Ok(req.req)
    }
}

struct FailingHandlerDouble;

#[async_trait::async_trait]
impl Handler for FailingHandlerDouble {
    type Request = TextPayload;
    type Response = TextPayload;

    async fn execute(
        &self,
        _req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Err(HandlerError::ExecutionFailed("boom".into()))
    }

    async fn health_check(
        &self,
        _req: HealthCheckRequest,
    ) -> Result<HealthCheckResponse, HandlerError> {
        Ok(HealthCheckResponse { healthy: false })
    }
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_execute_echo_returns_input_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = EchoHandlerDouble
        .execute(ExecutionRequest {
            req: TextPayload("payload".into()),
            ctx: &ctx,
        })
        .await;
    assert_eq!(result.unwrap(), TextPayload("payload".into()));
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_execute_failing_handler_returns_execution_failed_error() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = FailingHandlerDouble
        .execute(ExecutionRequest {
            req: TextPayload("x".into()),
            ctx: &ctx,
        })
        .await;
    assert!(matches!(result, Err(HandlerError::ExecutionFailed(_))));
}

/// @covers: Handler::id
#[test]
fn test_id_default_returns_handler_edge() {
    assert_eq!(EchoHandlerDouble.id(IdRequest).unwrap().id, "handler");
}

/// @covers: Handler::pattern
#[test]
fn test_pattern_default_returns_empty_string_edge() {
    assert_eq!(
        EchoHandlerDouble.pattern(PatternRequest).unwrap().pattern,
        ""
    );
}

/// @covers: Handler::health_check
#[tokio::test]
async fn test_health_check_default_returns_healthy_happy() {
    assert!(
        EchoHandlerDouble
            .health_check(HealthCheckRequest)
            .await
            .unwrap()
            .healthy
    );
}

/// @covers: Handler::health_check
#[tokio::test]
async fn test_health_check_overridden_returns_unhealthy_error() {
    assert!(
        !FailingHandlerDouble
            .health_check(HealthCheckRequest)
            .await
            .unwrap()
            .healthy
    );
}
