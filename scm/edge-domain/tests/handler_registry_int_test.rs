//! Integration tests for `HandlerRegistry`.
#![cfg(feature = "handler")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application::{Domain, Handler, HandlerError, HandlerRegistry};
use edge_application_handler::{
    DeregisterHandlerRequest, EmptinessRequest, ExecutionRequest, HandlerLookupRequest, IdRequest,
    IdResponse, LenRequest, ListIdsRequest, RegisterHandlerRequest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct EchoHandler {
    id: String,
}

#[async_trait]
impl Handler for EchoHandler {
    type Request = TextPayload;
    type Response = TextPayload;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.clone(),
        })
    }
    async fn execute(
        &self,
        req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Ok(req.req)
    }
}

fn echo(id: &str) -> Arc<dyn Handler<Request = TextPayload, Response = TextPayload>> {
    Arc::new(EchoHandler { id: id.to_string() })
}

fn registry() -> Arc<dyn HandlerRegistry<Request = TextPayload, Response = TextPayload>> {
    Domain.new_handler_registry()
}

/// @covers: HandlerRegistry — starts empty
#[test]
fn test_new_registry_is_empty() {
    let reg = registry();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
}

/// @covers: HandlerRegistry::register
#[test]
fn test_register_makes_handler_retrievable() {
    let reg = registry();
    reg.register(RegisterHandlerRequest::new(echo("svc")))
        .unwrap();
    assert!(
        reg.get(HandlerLookupRequest {
            id: "svc".to_string()
        })
        .unwrap()
        .handler
        .is_some(),
        "registered handler must be retrievable"
    );
}

/// @covers: HandlerRegistry::deregister
#[test]
fn test_deregister_removes_handler_and_returns_true() {
    let reg = registry();
    reg.register(RegisterHandlerRequest::new(echo("svc")))
        .unwrap();
    assert!(
        reg.deregister(DeregisterHandlerRequest {
            id: "svc".to_string()
        })
        .unwrap()
        .was_present
    );
    assert!(reg
        .get(HandlerLookupRequest {
            id: "svc".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}

/// @covers: HandlerRegistry::deregister — missing id returns false
#[test]
fn test_deregister_missing_handler_returns_false() {
    let reg = registry();
    assert!(
        !reg.deregister(DeregisterHandlerRequest {
            id: "ghost".to_string()
        })
        .unwrap()
        .was_present
    );
}

/// @covers: HandlerRegistry::list_ids
#[test]
fn test_list_ids_returns_all_registered_ids() {
    let reg = registry();
    reg.register(RegisterHandlerRequest::new(echo("a")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(echo("b")))
        .unwrap();
    let mut ids = reg.list_ids(ListIdsRequest).unwrap().ids;
    ids.sort();
    assert_eq!(ids, vec!["a", "b"]);
}

/// @covers: HandlerRegistry::len
#[test]
fn test_len_reflects_registration_count() {
    let reg = registry();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    reg.register(RegisterHandlerRequest::new(echo("a")))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    reg.register(RegisterHandlerRequest::new(echo("b")))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 2);
}

/// @covers: HandlerRegistry — registering same id replaces entry
#[test]
fn test_register_same_id_replaces_existing() {
    let reg = registry();
    reg.register(RegisterHandlerRequest::new(echo("svc")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(echo("svc")))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}
