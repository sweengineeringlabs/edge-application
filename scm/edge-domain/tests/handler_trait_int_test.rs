//! Integration tests for the `Handler` trait contract.
#![cfg(all(feature = "command", feature = "handler"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application::DirectCommandBusRequest;
use edge_application::DomainRuntime;
use edge_application::{Domain, Handler, HandlerContext, HandlerError};
use edge_application_handler::{ExecutionRequest, HealthCheckRequest, IdRequest, IdResponse};
use edge_application_observer::{ObserverContext, StdObserveFactory};
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IntPayload(u32);

impl edge_application_base::Request for IntPayload {}
impl edge_application_base::Response for IntPayload {}

struct Counter {
    id: String,
    calls: std::sync::atomic::AtomicUsize,
}

#[async_trait]
impl Handler for Counter {
    type Request = IntPayload;
    type Response = IntPayload;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.clone(),
        })
    }
    async fn execute(
        &self,
        req: ExecutionRequest<'_, IntPayload>,
    ) -> Result<IntPayload, HandlerError> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(IntPayload(req.req.0 * 2))
    }
}

struct SickHandler;
#[async_trait]
impl Handler for SickHandler {
    type Request = IntPayload;
    type Response = IntPayload;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "sick".to_string(),
        })
    }
    async fn execute(&self, _req: ExecutionRequest<'_, IntPayload>) -> Result<IntPayload, HandlerError> {
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
    bus: &'a dyn edge_application::CommandBus,
    observer: &'a dyn ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: bus,
        observer,
    }
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_handler_trait_execute_returns_transformed_value() {
    let h = Counter {
        id: "ctr".into(),
        calls: Default::default(),
    };
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, bus.as_ref(), observer.as_ref());
    let result = h
        .execute(ExecutionRequest {
            req: IntPayload(21),
            ctx: &ctx,
        })
        .await
        .unwrap();
    assert_eq!(result, IntPayload(42));
}

/// @covers: Handler::health_check — default is true
#[tokio::test]
async fn test_handler_trait_health_check_defaults_to_true() {
    let h = Counter {
        id: "ctr".into(),
        calls: Default::default(),
    };
    assert!(h.health_check(HealthCheckRequest).await.unwrap().healthy);
}

/// @covers: Handler::health_check — override to false
#[tokio::test]
async fn test_handler_trait_health_check_override_returns_false() {
    let h: Arc<dyn Handler<Request = IntPayload, Response = IntPayload>> = Arc::new(SickHandler);
    assert!(!h.health_check(HealthCheckRequest).await.unwrap().healthy);
}
