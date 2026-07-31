//! End-to-end contract tests for the `SecurityPrincipal` marker trait, exercised through a
//! test-double implementation via the crate's public API.

use std::sync::Arc;

use edge_application_handler::SecurityPrincipal;

struct PrincipalDouble;
impl SecurityPrincipal for PrincipalDouble {}

fn is_send_sync<T: Send + Sync>() -> bool {
    let _marker: std::marker::PhantomData<T> = std::marker::PhantomData;
    true
}

/// @covers: SecurityPrincipal
#[test]
fn test_security_principal_impl_is_send_and_sync_happy() {
    assert!(is_send_sync::<PrincipalDouble>());
}

/// @covers: SecurityPrincipal
#[test]
fn test_security_principal_coerces_to_trait_object_edge() {
    let p: Box<dyn SecurityPrincipal> = Box::new(PrincipalDouble);
    assert_eq!(
        std::mem::size_of_val(&*p),
        std::mem::size_of::<PrincipalDouble>()
    );
}

/// @covers: SecurityPrincipal
#[test]
fn test_security_principal_storable_in_arc_collection_error() {
    let principals: Vec<Arc<dyn SecurityPrincipal>> =
        vec![Arc::new(PrincipalDouble), Arc::new(PrincipalDouble)];
    assert_eq!(principals.len(), 2);
}
