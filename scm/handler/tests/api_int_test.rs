//! Layer-level coverage for the small request/response value types declared under
//! `api/handler/dto/` that have no dedicated per-type test file (SEA layer test
//! coverage, `sea_layer_test_coverage`). Each test constructs the type through the
//! crate's public API and asserts on its real shape or field values.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_handler::{
    DeregisterHandlerRequest, DeregisterHandlerResponse, EmptinessRequest, EmptinessResponse,
    ExecutionRequest, Handler, HandlerContext, HandlerError, HandlerLookupRequest,
    HandlerLookupResponse, HandlerRegistry, HealthCheckRequest, HealthCheckResponse, IdRequest,
    IdResponse, InProcessHandlerRegistry, LenRequest, LenResponse, ListIdsRequest,
    ListIdsResponse, PatternRequest, PatternResponse, RegisterHandlerRequest,
    RegisterHandlerResponse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct HandlerDouble;

#[async_trait::async_trait]
impl Handler for HandlerDouble {
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

// --- zero-sized marker request types ---

/// @covers: EmptinessRequest
#[test]
fn test_emptiness_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EmptinessRequest>(), 0);
    let _ = EmptinessRequest;
}

/// @covers: HealthCheckRequest
#[test]
fn test_health_check_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<HealthCheckRequest>(), 0);
    let _ = HealthCheckRequest;
}

/// @covers: IdRequest
#[test]
fn test_id_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<IdRequest>(), 0);
    let _ = IdRequest;
}

/// @covers: LenRequest
#[test]
fn test_len_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<LenRequest>(), 0);
    let _ = LenRequest;
}

/// @covers: ListIdsRequest
#[test]
fn test_list_ids_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ListIdsRequest>(), 0);
    let _ = ListIdsRequest;
}

/// @covers: PatternRequest
#[test]
fn test_pattern_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<PatternRequest>(), 0);
    let _ = PatternRequest;
}

/// @covers: RegisterHandlerResponse
#[test]
fn test_register_handler_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<RegisterHandlerResponse>(), 0);
    let _ = RegisterHandlerResponse;
}

// --- field-carrying request/response types ---

/// @covers: DeregisterHandlerRequest
#[test]
fn test_deregister_handler_request_holds_id_happy() {
    let r = DeregisterHandlerRequest {
        id: "h1".to_string(),
    };
    assert_eq!(r.id, "h1");
}

/// @covers: DeregisterHandlerResponse
#[test]
fn test_deregister_handler_response_holds_was_present_error() {
    let r = DeregisterHandlerResponse { was_present: false };
    assert!(!r.was_present);
}

/// @covers: EmptinessResponse
#[test]
fn test_emptiness_response_holds_empty_flag_happy() {
    let r = EmptinessResponse { empty: true };
    assert!(r.empty);
}

/// @covers: HealthCheckResponse
#[test]
fn test_health_check_response_holds_healthy_flag_happy() {
    let r = HealthCheckResponse { healthy: true };
    assert!(r.healthy);
}

/// @covers: IdResponse
#[test]
fn test_id_response_holds_id_happy() {
    let r = IdResponse {
        id: "abc".to_string(),
    };
    assert_eq!(r.id, "abc");
}

/// @covers: LenResponse
#[test]
fn test_len_response_holds_count_happy() {
    let r = LenResponse { count: 5 };
    assert_eq!(r.count, 5);
}

/// @covers: ListIdsResponse
#[test]
fn test_list_ids_response_holds_ids_happy() {
    let r = ListIdsResponse {
        ids: vec!["a".to_string(), "b".to_string()],
    };
    assert_eq!(r.ids, vec!["a".to_string(), "b".to_string()]);
}

/// @covers: PatternResponse
#[test]
fn test_pattern_response_holds_pattern_happy() {
    let r = PatternResponse {
        pattern: "/x".to_string(),
    };
    assert_eq!(r.pattern, "/x");
}

/// @covers: HandlerLookupRequest
#[test]
fn test_handler_lookup_request_holds_id_happy() {
    let r = HandlerLookupRequest {
        id: "h2".to_string(),
    };
    assert_eq!(r.id, "h2");
}

/// @covers: HandlerLookupResponse
#[test]
fn test_handler_lookup_response_holds_none_when_absent_edge() {
    let r: HandlerLookupResponse<TextPayload, TextPayload> = HandlerLookupResponse { handler: None };
    assert!(r.handler.is_none());
}

/// @covers: HandlerLookupResponse
#[test]
fn test_handler_lookup_response_holds_some_handler_happy() {
    let handler: Arc<dyn Handler<Request = TextPayload, Response = TextPayload>> =
        Arc::new(HandlerDouble);
    let r = HandlerLookupResponse {
        handler: Some(handler),
    };
    assert_eq!(r.handler.unwrap().id(IdRequest).unwrap().id, "stub");
}

/// @covers: RegisterHandlerRequest
#[test]
fn test_register_handler_request_new_wraps_handler_happy() {
    let req = RegisterHandlerRequest::new(Arc::new(HandlerDouble)
        as Arc<dyn Handler<Request = TextPayload, Response = TextPayload>>);
    let reg = InProcessHandlerRegistry::<TextPayload, TextPayload>::default();
    reg.register(req).unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}

/// @covers: ExecutionRequest
#[test]
fn test_execution_request_holds_req_and_ctx_happy() {
    use edge_application_command::NoopCommandBus;
    use edge_application_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;

    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &NoopCommandBus,
        observer: observer.as_ref(),
    };
    let req = ExecutionRequest {
        req: "payload".to_string(),
        ctx: &ctx,
    };
    assert_eq!(req.req, "payload");
}
