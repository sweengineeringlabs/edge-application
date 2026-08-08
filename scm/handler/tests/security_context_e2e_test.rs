//! End-to-end contract tests for `SecurityContext`, exercised through the crate's public API.
//! `edge_application_handler::SecurityContext` re-exports `edge-application-base`'s re-export
//! of `edge_security_runtime::SecurityContext` directly — no local mirror trait (issue #152).

use std::sync::Arc;

use edge_application_handler::SecurityContext;

fn is_send_sync<T: Send + Sync>() -> bool {
    let _marker: std::marker::PhantomData<T> = std::marker::PhantomData;
    true
}

/// @covers: SecurityContext
#[test]
fn test_security_context_is_send_and_sync_happy() {
    assert!(is_send_sync::<SecurityContext>());
}

/// @covers: SecurityContext
#[test]
fn test_security_context_usable_by_reference_edge() {
    let ctx = SecurityContext::unauthenticated();
    let r: &SecurityContext = &ctx;
    assert!(!r.authenticated);
}

/// @covers: SecurityContext
#[test]
fn test_security_context_storable_in_arc_collection_error() {
    let contexts: Vec<Arc<SecurityContext>> = vec![
        Arc::new(SecurityContext::unauthenticated()),
        Arc::new(SecurityContext::unauthenticated()),
    ];
    assert_eq!(contexts.len(), 2);
}
