#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Handler trait is exported from the crate root.
#![cfg(all(feature = "command", feature = "handler"))]

use async_trait::async_trait;
use edge_application::DirectCommandBusRequest;
use edge_application::Domain;
use edge_application::DomainRuntime;
use edge_application::Handler;
use edge_application::HandlerContext;
use edge_application::HandlerError;
use edge_application_handler::{ExecutionRequest, HealthCheckRequest, IdRequest, IdResponse};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IntPayload(i32);

impl edge_application_base::Request for IntPayload {}
impl edge_application_base::Response for IntPayload {}

struct Doubler;

#[async_trait]
impl Handler for Doubler {
    type Request = IntPayload;
    type Response = IntPayload;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "doubler".to_string(),
        })
    }
    async fn execute(
        &self,
        req: ExecutionRequest<'_, IntPayload>,
    ) -> Result<IntPayload, HandlerError> {
        Ok(IntPayload(req.req.0 * 2))
    }
}

#[tokio::test]
async fn test_handler_svc_facade_execute_doubles_input() {
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
    assert_eq!(
        Doubler
            .execute(ExecutionRequest {
                req: IntPayload(21),
                ctx: &ctx
            })
            .await
            .unwrap(),
        IntPayload(42)
    );
}

#[tokio::test]
async fn test_handler_svc_facade_health_check_defaults_true() {
    assert!(
        Doubler
            .health_check(HealthCheckRequest)
            .await
            .unwrap()
            .healthy
    );
}
