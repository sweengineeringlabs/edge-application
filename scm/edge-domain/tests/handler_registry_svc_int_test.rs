#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — HandlerRegistry is exported from the crate root.
#![cfg(feature = "handler")]

use edge_application::Domain;
use edge_application::EchoHandler;
use edge_application::HandlerRegistry;
use edge_application_handler::{HandlerLookupRequest, RegisterHandlerRequest};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

#[test]
fn test_handler_registry_svc_facade_register_and_retrieve() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    let handler: Arc<dyn edge_application::Handler<Request = TextPayload, Response = TextPayload>> =
        Arc::new(EchoHandler::from(("echo", "*")));
    reg.register(RegisterHandlerRequest::new(handler)).unwrap();
    assert!(reg
        .get(HandlerLookupRequest {
            id: "echo".to_string()
        })
        .unwrap()
        .handler
        .is_some());
}

#[test]
fn test_handler_registry_svc_facade_missing_id_returns_none() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg
        .get(HandlerLookupRequest {
            id: "absent".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}
