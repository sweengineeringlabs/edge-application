//! Integration tests for the `edge_application::Authenticator`/`Authorizer` re-exports.
#![cfg(feature = "security")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    AnonymousPrincipal, AuthenticateRequest, Authenticator, AuthnContext, AuthnError,
    AuthorizeRequest, Authorizer, AuthzContext, AuthzError, SecurityContext,
};
use edge_security_authn::Authenticator as RawAuthenticator;
use edge_security_authz::Authorizer as RawAuthorizer;

struct AlwaysAuthenticates;

#[async_trait::async_trait]
impl Authenticator for AlwaysAuthenticates {
    async fn authenticate(&self, req: AuthenticateRequest<'_>) -> Result<(), AuthnError> {
        req.ctx.set_authenticated(true);
        req.ctx.set_principal(Box::new(AnonymousPrincipal));
        Ok(())
    }
}

struct RejectsUnlessBearerPresent;

#[async_trait::async_trait]
impl Authenticator for RejectsUnlessBearerPresent {
    async fn authenticate(&self, req: AuthenticateRequest<'_>) -> Result<(), AuthnError> {
        // AuthnContext doesn't expose SecurityContext::token directly (by design,
        // it only surfaces the fields authenticators actually read/write) — a
        // bearer credential's presence is checked via metadata instead.
        if !req.ctx.metadata().contains_key("authorization") {
            return Err(AuthnError::MissingToken);
        }
        req.ctx.set_authenticated(true);
        Ok(())
    }
}

/// @covers: Authenticator::authenticate
#[tokio::test]
async fn test_authenticate_always_authenticates_marks_context_happy() {
    let mut ctx = AuthnContext::new(SecurityContext::unauthenticated());
    AlwaysAuthenticates
        .authenticate(AuthenticateRequest { ctx: &mut ctx })
        .await
        .unwrap();
    assert!(ctx.authenticated());
    assert!(ctx.principal().is_some());
}

/// @covers: Authenticator::authenticate
#[tokio::test]
async fn test_authenticate_missing_token_returns_error_error() {
    let mut ctx = AuthnContext::new(SecurityContext::unauthenticated());
    let result = RejectsUnlessBearerPresent
        .authenticate(AuthenticateRequest { ctx: &mut ctx })
        .await;
    assert!(matches!(result, Err(AuthnError::MissingToken)));
}

/// @covers: Authenticator::authenticate
#[tokio::test]
async fn test_authenticate_with_token_present_succeeds_edge() {
    let mut ctx = AuthnContext::new(SecurityContext::unauthenticated());
    ctx.metadata_mut()
        .insert("authorization".to_string(), "Bearer valid-token".to_string());
    RejectsUnlessBearerPresent
        .authenticate(AuthenticateRequest { ctx: &mut ctx })
        .await
        .unwrap();
    assert!(ctx.authenticated());
}

struct AlwaysAuthorizes;

#[async_trait::async_trait]
impl Authorizer for AlwaysAuthorizes {
    async fn authorize(&self, req: AuthorizeRequest<'_>) -> Result<(), AuthzError> {
        req.ctx.set_is_authorized(true);
        Ok(())
    }
}

struct RequiresAuthenticatedPrincipal;

#[async_trait::async_trait]
impl Authorizer for RequiresAuthenticatedPrincipal {
    async fn authorize(&self, req: AuthorizeRequest<'_>) -> Result<(), AuthzError> {
        if req.ctx.principal().is_none() {
            return Err(AuthzError::MissingPrincipal);
        }
        req.ctx.set_is_authorized(true);
        Ok(())
    }
}

/// @covers: Authorizer::authorize
#[tokio::test]
async fn test_authorize_always_authorizes_marks_context_happy() {
    let mut ctx = AuthzContext::new(SecurityContext::unauthenticated());
    AlwaysAuthorizes
        .authorize(AuthorizeRequest { ctx: &mut ctx })
        .await
        .unwrap();
    assert!(ctx.is_authorized());
}

/// @covers: Authorizer::authorize
#[tokio::test]
async fn test_authorize_missing_principal_returns_error_error() {
    let mut ctx = AuthzContext::new(SecurityContext::unauthenticated());
    let result = RequiresAuthenticatedPrincipal
        .authorize(AuthorizeRequest { ctx: &mut ctx })
        .await;
    assert!(matches!(result, Err(AuthzError::MissingPrincipal)));
}

/// @covers: Authorizer::authorize
#[tokio::test]
async fn test_authorize_with_principal_present_succeeds_edge() {
    let mut ctx = AuthzContext::new(SecurityContext::authenticated_with(Box::new(
        AnonymousPrincipal,
    )));
    RequiresAuthenticatedPrincipal
        .authorize(AuthorizeRequest { ctx: &mut ctx })
        .await
        .unwrap();
    assert!(ctx.is_authorized());
}

/// @covers: edge_application's re-exported Authenticator/Authorizer are the underlying
/// edge_security_authn/edge_security_authz traits, not look-alike wrappers.
#[test]
fn test_facade_authenticator_authorizer_are_the_raw_traits_edge() {
    fn assert_authenticator<T: Authenticator + RawAuthenticator>() -> bool {
        true
    }
    fn assert_authorizer<T: Authorizer + RawAuthorizer>() -> bool {
        true
    }
    assert!(assert_authenticator::<AlwaysAuthenticates>());
    assert!(assert_authorizer::<AlwaysAuthorizes>());
}
