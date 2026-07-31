//! Integration tests — `InProcessHandlerRegistry` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DeregisterHandlerRequest, EmptinessRequest, ExecutionRequest, Handler, HandlerContext,
    HandlerError, HandlerLookupRequest, HandlerRegistry, IdRequest, IdResponse,
    InProcessHandlerRegistry, LenRequest, ListIdsRequest, RegisterHandlerRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct Stub {
    id: &'static str,
    response: &'static str,
}

#[async_trait]
impl Handler for Stub {
    type Request = TextPayload;
    type Response = TextPayload;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.to_string(),
        })
    }
    async fn execute(
        &self,
        _req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Ok(TextPayload(self.response.into()))
    }
}

fn make_reg() -> InProcessHandlerRegistry<TextPayload, TextPayload> {
    InProcessHandlerRegistry::default()
}

/// @covers: InProcessHandlerRegistry::new — creates empty registry
#[test]
fn test_new_creates_empty_registry_happy() {
    let reg = make_reg();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
}

/// @covers: InProcessHandlerRegistry default
#[test]
fn test_default_creates_empty_registry_edge() {
    let reg = InProcessHandlerRegistry::<TextPayload, TextPayload>::default();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: InProcessHandlerRegistry::register — get returns Some
#[test]
fn test_register_makes_handler_retrievable_happy() {
    let reg = make_reg();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "s1",
        response: "r1",
    })))
    .unwrap();
    let handler = reg
        .get(HandlerLookupRequest {
            id: "s1".to_string(),
        })
        .unwrap()
        .handler;
    assert!(handler.is_some());
    assert_eq!(handler.unwrap().id(IdRequest).unwrap().id, "s1");
}

/// @covers: InProcessHandlerRegistry::register — duplicate id replaces
#[test]
fn test_register_duplicate_id_replaces_handler_edge() {
    let reg = make_reg();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "dup",
        response: "first",
    })))
    .unwrap();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "dup",
        response: "second",
    })))
    .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    let h = reg
        .get(HandlerLookupRequest {
            id: "dup".to_string(),
        })
        .unwrap()
        .handler
        .unwrap();
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    assert_eq!(
        block_on(h.execute(ExecutionRequest {
            req: TextPayload("".into()),
            ctx: &ctx
        }))
        .unwrap(),
        TextPayload("second".into())
    );
}

/// @covers: InProcessHandlerRegistry::deregister — returns true for existing id
#[test]
fn test_deregister_existing_returns_true_happy() {
    let reg = make_reg();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "to-remove",
        response: "x",
    })))
    .unwrap();
    assert!(
        reg.deregister(DeregisterHandlerRequest {
            id: "to-remove".to_string()
        })
        .unwrap()
        .was_present
    );
    assert!(reg
        .get(HandlerLookupRequest {
            id: "to-remove".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}

/// @covers: InProcessHandlerRegistry::deregister — returns false for missing
#[test]
fn test_deregister_missing_returns_false_error() {
    let reg = make_reg();
    assert!(
        !reg.deregister(DeregisterHandlerRequest {
            id: "missing".to_string()
        })
        .unwrap()
        .was_present
    );
}

/// @covers: InProcessHandlerRegistry::list_ids — sorted
#[test]
fn test_list_ids_returns_sorted_ids_happy() {
    let reg = make_reg();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "z",
        response: "",
    })))
    .unwrap();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "a",
        response: "",
    })))
    .unwrap();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "m",
        response: "",
    })))
    .unwrap();
    let ids = reg.list_ids(ListIdsRequest).unwrap().ids;
    assert_eq!(ids, vec!["a", "m", "z"]);
}

/// @covers: InProcessHandlerRegistry::list_ids — empty registry
#[test]
fn test_list_ids_empty_registry_returns_empty_vec_edge() {
    let reg = make_reg();
    assert!(reg.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

/// @covers: retrieved handler executes correctly
#[test]
fn test_retrieved_handler_produces_expected_response_happy() {
    let reg = make_reg();
    reg.register(RegisterHandlerRequest::new(Arc::new(Stub {
        id: "exec",
        response: "pong",
    })))
    .unwrap();
    let h = reg
        .get(HandlerLookupRequest {
            id: "exec".to_string(),
        })
        .unwrap()
        .handler
        .unwrap();
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    assert_eq!(
        block_on(h.execute(ExecutionRequest {
            req: TextPayload("ping".into()),
            ctx: &ctx
        }))
        .unwrap(),
        TextPayload("pong".into())
    );
}
