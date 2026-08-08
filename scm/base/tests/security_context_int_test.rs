//! Integration tests for `edge_application_base::SecurityContext` — a direct re-export of
//! `edge_security_runtime::SecurityContext`, not a locally mirrored trait/bridge.

use edge_application_base::SecurityContext;

/// @covers: SecurityContext — the re-export resolves to the real, external type (not a local
/// wrapper) -- `edge_application_base::SecurityContext` and `edge_security_runtime::SecurityContext`
/// must be the exact same type.
#[test]
fn test_security_context_reexport_matches_source_crate_type_happy() {
    assert_eq!(
        std::any::TypeId::of::<SecurityContext>(),
        std::any::TypeId::of::<edge_security_runtime::SecurityContext>()
    );
}

/// @covers: SecurityContext::unauthenticated — usable end-to-end through the re-exported path
#[test]
fn test_security_context_unauthenticated_usable_via_reexport_happy() {
    let ctx = SecurityContext::unauthenticated();
    assert!(!ctx.authenticated);
    assert!(ctx.principal.is_none());
}

/// @covers: SecurityContext::with_tenant/with_claim — builder chain works through the re-export
#[test]
fn test_security_context_builder_chain_via_reexport_happy() {
    let ctx = SecurityContext::unauthenticated()
        .with_tenant("acme")
        .with_claim("role", "admin");

    assert_eq!(ctx.tenant_id.as_deref(), Some("acme"));
    assert_eq!(ctx.claim("role"), Some("admin"));
}

/// @covers: SecurityContext::claim — absent key returns None through the re-export, same as the
/// source crate's own behavior
#[test]
fn test_security_context_claim_returns_none_for_absent_key_error() {
    let ctx = SecurityContext::unauthenticated();
    assert_eq!(ctx.claim("missing"), None);
}

/// @covers: SecurityContext — usable as a function parameter across a boundary, by reference
#[test]
fn test_security_context_usable_by_reference_across_boundary_edge() {
    fn accepts(ctx: &SecurityContext) -> bool {
        ctx.authenticated
    }

    let ctx = SecurityContext::unauthenticated();
    assert!(!accepts(&ctx));
}
