//! Handler registry — register, resolve, execute, and deregister domain Handlers.
//!
//! Run:
//!     cargo run -p edge-domain --example handler_registry
//!
//! Demonstrates the full domain execution-unit contract:
//!   new_handler_registry → register → get → execute → health_check → deregister
//!
//! SEA constraint: all imports come from the `edge_application` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application::DirectCommandBusRequest;
use edge_application::DomainRuntime;
use edge_application::{Domain, Handler, HandlerContext, HandlerError};
use edge_application_handler::{
    DeregisterHandlerRequest, EmptinessRequest, ExecutionRequest, HandlerLookupRequest, IdRequest,
    IdResponse, ListIdsRequest, RegisterHandlerRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct GreetHandler;

#[async_trait]
impl Handler for GreetHandler {
    type Request = TextPayload;
    type Response = TextPayload;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "greet".to_string(),
        })
    }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        if req.req.0.is_empty() {
            return Err(HandlerError::InvalidRequest(
                "name must not be empty".into(),
            ));
        }
        Ok(TextPayload(format!("Hello, {}!", req.req.0)))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(registry.is_empty(EmptinessRequest)?.empty);

    registry.register(RegisterHandlerRequest::new(Arc::new(GreetHandler)))?;
    println!("registered:   {:?}", registry.list_ids(ListIdsRequest)?.ids);

    let handler = registry
        .get(HandlerLookupRequest {
            id: "greet".to_string(),
        })?
        .handler
        .expect("handler must be present");
    let security = SecurityContext::unauthenticated();
    let bus = Domain.direct_command_bus(DirectCommandBusRequest)?.bus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: bus.as_ref(),
        observer: observer.as_ref(),
    };

    let resp = handler
        .execute(ExecutionRequest {
            req: TextPayload("world".into()),
            ctx: &ctx,
        })
        .await?;
    println!("execute       → {}", resp.0);

    let err = handler
        .execute(ExecutionRequest {
            req: TextPayload("".into()),
            ctx: &ctx,
        })
        .await
        .unwrap_err();
    println!("empty name    → {err}");

    let healthy = handler
        .health_check(edge_application_handler::HealthCheckRequest)
        .await?
        .healthy;
    println!("health_check  → {healthy}");
    assert!(healthy);

    let removed = registry
        .deregister(DeregisterHandlerRequest {
            id: "greet".to_string(),
        })?
        .was_present;
    assert!(removed);
    assert!(registry
        .get(HandlerLookupRequest {
            id: "greet".to_string()
        })?
        .handler
        .is_none());
    println!("after remove: {:?}", registry.list_ids(ListIdsRequest)?.ids);

    Ok(())
}
