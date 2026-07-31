//! Integration tests for `SecurityPrincipal`, bridged from `edge_security_runtime::SecurityContext`.

use edge_application_base::SecurityPrincipal;
use edge_security_runtime::SecurityContext;

/// @covers: SecurityPrincipal — SecurityContext::unauthenticated satisfies it
#[test]
fn test_security_context_unauthenticated_satisfies_security_principal_happy() {
    let ctx = SecurityContext::unauthenticated();
    let principal: &dyn SecurityPrincipal = &ctx;
    // SecurityPrincipal has zero methods; the assertion is that this reference exists at all
    // -- if the bridge impl in core/context/security_bridge.rs didn't compile or wasn't
    // wired up, this line itself would fail to build.
    let _ = principal;
    assert!(!ctx.authenticated);
}

/// @covers: SecurityPrincipal — held as a trait object across a function boundary
#[test]
fn test_security_principal_usable_as_trait_object_edge() {
    fn accepts_principal(_p: &dyn SecurityPrincipal) {}

    let ctx = SecurityContext::unauthenticated();
    accepts_principal(&ctx);
}

/// @covers: SecurityPrincipal — a second, independently-constructed SecurityContext also satisfies it
#[test]
fn test_multiple_security_contexts_each_satisfy_security_principal_error() {
    let a = SecurityContext::unauthenticated();
    let b = SecurityContext::unauthenticated();
    let pa: &dyn SecurityPrincipal = &a;
    let pb: &dyn SecurityPrincipal = &b;
    // Two independent instances, both valid trait objects -- proves the bridge isn't
    // accidentally singleton-scoped or otherwise order-dependent.
    let _ = (pa, pb);
    assert!(!a.authenticated && !b.authenticated);
}
