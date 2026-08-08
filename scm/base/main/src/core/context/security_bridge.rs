//! Bridge from `edge_security_runtime::SecurityContext` to [`SecurityPrincipal`].
//!
//! `SecurityPrincipal` is declared in `api/` (SEA `no_foreign_type` forbids naming
//! `edge_security_runtime::SecurityContext` there directly); the concrete bridge to the
//! real, external principal type lives here in `core/`, same pattern used throughout this
//! workspace for foreign-type decoupling.

use edge_security_runtime::{PrincipalRequest, SecurityContext};

use crate::api::SecurityPrincipal;

impl SecurityPrincipal for SecurityContext {
    fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    fn subject(&self) -> Option<String> {
        self.principal
            .as_ref()
            .and_then(|p| p.id(PrincipalRequest).ok())
            .map(|resp| resp.value)
    }

    fn tenant_id(&self) -> Option<&str> {
        self.tenant_id.as_deref()
    }

    fn claim(&self, key: &str) -> Option<&str> {
        SecurityContext::claim(self, key)
    }
}

#[cfg(test)]
mod tests {
    use edge_security_runtime::{AnonymousPrincipal, SecurityContext, SimplePrincipal};

    use super::*;

    /// @covers: is_authenticated
    #[test]
    fn test_is_authenticated_reflects_context_state_happy() {
        let unauth = SecurityContext::unauthenticated();
        let auth = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));

        assert!(!SecurityPrincipal::is_authenticated(&unauth));
        assert!(SecurityPrincipal::is_authenticated(&auth));
    }

    /// @covers: subject
    #[test]
    fn test_subject_returns_principal_id_when_present_happy() {
        let ctx = SecurityContext::authenticated_with(Box::new(SimplePrincipal::new(
            "user-42", "tenant",
        )));

        assert_eq!(SecurityPrincipal::subject(&ctx), Some("user-42".to_string()));
    }

    /// @covers: subject
    #[test]
    fn test_subject_returns_none_when_no_principal_error() {
        let ctx = SecurityContext::unauthenticated();

        assert_eq!(SecurityPrincipal::subject(&ctx), None);
    }

    /// @covers: tenant_id
    #[test]
    fn test_tenant_id_returns_stored_value_happy() {
        let ctx = SecurityContext::unauthenticated().with_tenant("acme");

        assert_eq!(SecurityPrincipal::tenant_id(&ctx), Some("acme"));
    }

    /// @covers: claim
    #[test]
    fn test_claim_round_trips_through_principal_trait_happy() {
        let ctx = SecurityContext::unauthenticated().with_claim("role", "admin");

        assert_eq!(SecurityPrincipal::claim(&ctx, "role"), Some("admin"));
    }

    /// @covers: claim
    #[test]
    fn test_claim_returns_none_for_absent_key_error() {
        let ctx = SecurityContext::unauthenticated();

        assert_eq!(SecurityPrincipal::claim(&ctx, "missing"), None);
    }
}
