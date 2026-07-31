//! End-to-end contract tests for the `HandlerRegistry` trait, exercised through the
//! crate's canonical [`InProcessHandlerRegistry`] implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_handler::{
    DeregisterHandlerRequest, EmptinessRequest, ExecutionRequest, Handler, HandlerError,
    HandlerLookupRequest, HandlerRegistry, IdRequest, IdResponse, InProcessHandlerRegistry,
    LenRequest, ListIdsRequest, RegisterHandlerRequest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct StubHandler;

#[async_trait::async_trait]
impl Handler for StubHandler {
    type Request = TextPayload;
    type Response = TextPayload;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "stub".to_string(),
        })
    }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Ok(req.req)
    }
}

fn make_registry() -> InProcessHandlerRegistry<TextPayload, TextPayload> {
    InProcessHandlerRegistry::default()
}

/// @covers: HandlerRegistry::register
#[test]
fn test_register_makes_handler_retrievable_happy() {
    let reg = make_registry();
    reg.register(RegisterHandlerRequest::new(Arc::new(StubHandler)))
        .unwrap();
    let found = reg
        .get(HandlerLookupRequest {
            id: "stub".to_string(),
        })
        .unwrap()
        .handler;
    assert!(found.is_some());
}

/// @covers: HandlerRegistry::deregister
#[test]
fn test_deregister_missing_id_returns_false_error() {
    let reg = make_registry();
    let result = reg.deregister(DeregisterHandlerRequest {
        id: "missing".to_string(),
    });
    assert!(!result.unwrap().was_present);
}

/// @covers: HandlerRegistry::get
#[test]
fn test_get_missing_id_returns_none_edge() {
    let reg = make_registry();
    let result = reg.get(HandlerLookupRequest {
        id: "missing".to_string(),
    });
    assert!(result.unwrap().handler.is_none());
}

/// @covers: HandlerRegistry::list_ids
#[test]
fn test_list_ids_reflects_registered_handlers_happy() {
    let reg = make_registry();
    reg.register(RegisterHandlerRequest::new(Arc::new(StubHandler)))
        .unwrap();
    assert_eq!(
        reg.list_ids(ListIdsRequest).unwrap().ids,
        vec!["stub".to_string()]
    );
}

/// @covers: HandlerRegistry::len
#[test]
fn test_len_counts_registered_handlers_happy() {
    let reg = make_registry();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    reg.register(RegisterHandlerRequest::new(Arc::new(StubHandler)))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}

/// @covers: HandlerRegistry::is_empty
#[test]
fn test_is_empty_default_method_reflects_len_edge() {
    let reg = make_registry();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    reg.register(RegisterHandlerRequest::new(Arc::new(StubHandler)))
        .unwrap();
    assert!(!reg.is_empty(EmptinessRequest).unwrap().empty);
}
