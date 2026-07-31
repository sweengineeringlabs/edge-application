//! `SecurityPrincipal` — the authenticated request principal shape shared with `HandlerContext`.

/// Opaque handle to the authenticated (or unauthenticated) principal for a request.
///
/// Zero methods by design -- callers hold and pass the reference, they don't call anything
/// on it directly. The real principal type, `edge_security_runtime::SecurityContext`, is
/// external to this workspace; `impl SecurityPrincipal for SecurityContext {}` lives in
/// `core/` (see `no_foreign_type`), not here.
pub trait SecurityPrincipal: Send + Sync {}
