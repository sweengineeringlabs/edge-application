//! Integration tests — `HandlerRegistry` trait via `InProcessHandlerRegistry`.
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

struct Fixed {
    id: &'static str,
}

#[async_trait]
impl Handler for Fixed {
    type Request = TextPayload;
    type Response = TextPayload;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.to_string(),
        })
    }
    async fn execute(
        &self,
        req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        Ok(req.req)
    }
}

fn make_reg() -> InProcessHandlerRegistry<TextPayload, TextPayload> {
    InProcessHandlerRegistry::default()
}

fn reg_id(id: &'static str) -> RegisterHandlerRequest<TextPayload, TextPayload> {
    RegisterHandlerRequest::new(Arc::new(Fixed { id }))
}

fn dereg(id: &str) -> DeregisterHandlerRequest {
    DeregisterHandlerRequest { id: id.to_string() }
}

fn getreq(id: &str) -> HandlerLookupRequest {
    HandlerLookupRequest { id: id.to_string() }
}

/// @covers: HandlerRegistry::register — handler is retrievable after registration
#[test]
fn test_register_handler_can_be_retrieved_happy() {
    let reg = make_reg();
    reg.register(reg_id("alpha")).unwrap();
    let handler = reg.get(getreq("alpha")).unwrap().handler;
    assert!(handler.is_some());
    assert_eq!(handler.unwrap().id(IdRequest).unwrap().id, "alpha");
}

/// @covers: HandlerRegistry::register — replaces duplicate id
#[test]
fn test_register_duplicate_id_replaces_existing_edge() {
    let reg = make_reg();
    reg.register(reg_id("alpha")).unwrap();
    reg.register(reg_id("alpha")).unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}

/// @covers: HandlerRegistry::deregister — existing handler
#[test]
fn test_deregister_existing_handler_returns_true_happy() {
    let reg = make_reg();
    reg.register(reg_id("beta")).unwrap();
    assert!(reg.deregister(dereg("beta")).unwrap().was_present);
    assert!(reg.get(getreq("beta")).unwrap().handler.is_none());
}

/// @covers: HandlerRegistry::deregister — missing id returns false
#[test]
fn test_deregister_missing_id_returns_false_error() {
    let reg = make_reg();
    assert!(!reg.deregister(dereg("ghost")).unwrap().was_present);
}

/// @covers: HandlerRegistry::get — missing id
#[test]
fn test_get_missing_id_returns_none_error() {
    let reg = make_reg();
    assert!(reg.get(getreq("missing")).unwrap().handler.is_none());
}

/// @covers: HandlerRegistry::list_ids
#[test]
fn test_list_ids_returns_registered_ids_happy() {
    let reg = make_reg();
    reg.register(reg_id("b")).unwrap();
    reg.register(reg_id("a")).unwrap();
    let mut ids = reg.list_ids(ListIdsRequest).unwrap().ids;
    ids.sort();
    assert_eq!(ids, vec!["a", "b"]);
}

/// @covers: HandlerRegistry::list_ids — empty registry
#[test]
fn test_list_ids_empty_registry_returns_empty_edge() {
    let reg = make_reg();
    assert!(reg.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

/// @covers: HandlerRegistry::len — counts correctly
#[test]
fn test_len_reflects_current_count_happy() {
    let reg = make_reg();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    reg.register(reg_id("x")).unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    reg.deregister(dereg("x")).unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
}

/// @covers: HandlerRegistry::is_empty — new registry
#[test]
fn test_is_empty_new_registry_returns_true_edge() {
    let reg = make_reg();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: retrieved handler executes correctly
#[test]
fn test_retrieved_handler_executes_correctly_happy() {
    let reg = make_reg();
    reg.register(reg_id("exec")).unwrap();
    let h = reg.get(getreq("exec")).unwrap().handler.unwrap();
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
            req: TextPayload("data".into()),
            ctx: &ctx
        }))
        .unwrap(),
        TextPayload("data".into())
    );
}

/// @covers: HandlerRegistry::register — registering with a duplicate id does not produce an error
#[test]
fn test_register_duplicate_id_is_not_an_error_error() {
    let reg = make_reg();
    reg.register(reg_id("dup")).unwrap();
    reg.register(reg_id("dup")).unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}

/// @covers: HandlerRegistry::deregister — re-deregistering already-removed id
#[test]
fn test_deregister_already_removed_id_returns_false_edge() {
    let reg = make_reg();
    reg.register(reg_id("once")).unwrap();
    assert!(reg.deregister(dereg("once")).unwrap().was_present);
    assert!(!reg.deregister(dereg("once")).unwrap().was_present);
}

/// @covers: HandlerRegistry::get — retrieve a known registered handler
#[test]
fn test_get_registered_handler_returns_some_happy() {
    let reg = make_reg();
    reg.register(reg_id("present")).unwrap();
    let handler = reg.get(getreq("present")).unwrap().handler;
    assert!(handler.is_some());
    assert_eq!(handler.unwrap().id(IdRequest).unwrap().id, "present");
}

/// @covers: HandlerRegistry::get — re-deregistered handler is gone
#[test]
fn test_get_after_deregister_returns_none_edge() {
    let reg = make_reg();
    reg.register(reg_id("gone")).unwrap();
    reg.deregister(dereg("gone")).unwrap();
    assert!(reg.get(getreq("gone")).unwrap().handler.is_none());
}

/// @covers: HandlerRegistry::list_ids — no error path; empty registry returns empty vec
#[test]
fn test_list_ids_on_empty_registry_returns_empty_vec_error() {
    let reg = make_reg();
    assert!(reg.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

/// @covers: HandlerRegistry::len — infallible; zero is valid, not an error
#[test]
fn test_len_zero_after_all_deregistered_is_not_error_error() {
    let reg = make_reg();
    reg.register(reg_id("ephemeral")).unwrap();
    reg.deregister(dereg("ephemeral")).unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
}

/// @covers: HandlerRegistry::len — count after multiple registers equals distinct ids
#[test]
fn test_len_after_two_distinct_registers_is_two_edge() {
    let reg = make_reg();
    reg.register(reg_id("p")).unwrap();
    reg.register(reg_id("q")).unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 2);
}

/// @covers: HandlerRegistry::is_empty — empty registry returns true
#[test]
fn test_is_empty_on_empty_registry_returns_true_happy() {
    let reg = make_reg();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: HandlerRegistry::is_empty — non-empty registry never returns true as an error
#[test]
fn test_is_empty_after_register_returns_false_not_an_error_error() {
    let reg = make_reg();
    reg.register(reg_id("item")).unwrap();
    assert!(!reg.is_empty(EmptinessRequest).unwrap().empty);
}
