//! Integration tests — `edge_application_base::{Request, Response}` exercised through
//! `edge-domain`'s own factory methods (`Domain::echo_handler`, `Domain::new_handler_registry`).
#![cfg(feature = "handler")]

use edge_application::Domain;
use edge_application_base::{Request, Response};
use edge_application_handler::EmptinessRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ping(String);

impl Request for Ping {}
impl Response for Ping {}

/// @covers: edge_application_base::Request, edge_application_base::Response — satisfied via echo_handler
#[test]
fn test_echo_handler_accepts_base_request_response_impl_happy() {
    let h = Domain.echo_handler::<Ping>("echo", "/ping");
    assert_eq!(h.id(edge_application_handler::IdRequest).unwrap().id, "echo");
}

/// @covers: edge_application_base::Request, edge_application_base::Response — satisfied via new_handler_registry
#[test]
fn test_new_handler_registry_accepts_base_request_response_impl_edge() {
    let registry = Domain.new_handler_registry::<Ping, Ping>();
    assert!(registry.is_empty(EmptinessRequest).unwrap().empty);
}
