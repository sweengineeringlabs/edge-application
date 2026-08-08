//! `SecurityPrincipal` — the authenticated request principal shape shared with `HandlerContext`.

/// Handle to the authenticated (or unauthenticated) principal for a request.
///
/// The real principal type, `edge_security_runtime::SecurityContext`, is external to this
/// workspace; `impl SecurityPrincipal for SecurityContext` lives in `core/` (see
/// `no_foreign_type`), not here. All methods here return primitives only (`bool`, `&str`,
/// `Option<String>`) so this trait never has to name that foreign type.
pub trait SecurityPrincipal: Send + Sync {
    /// Whether this principal represents an authenticated caller.
    fn is_authenticated(&self) -> bool {
        false
    }

    /// The resolved identity string for this principal (e.g. subject/user id), if any.
    fn subject(&self) -> Option<String> {
        None
    }

    /// The tenant scope for this request, for multi-tenant deployments.
    fn tenant_id(&self) -> Option<&str> {
        None
    }

    /// Retrieve a claim value by key (e.g. JWT claims, propagated headers).
    fn claim(&self, key: &str) -> Option<&str> {
        let _ = key;
        None
    }
}
