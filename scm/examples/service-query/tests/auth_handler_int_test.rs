//! Integration test: a single `Handler::execute()` call that both reads `HandlerContext` for
//! real and, as part of the same call, dispatches a real `Query` — `Handler` -> `Query`,
//! connected and working, as one observable call chain.
//!
//! `AuthHandler` holds its own constructor-injected `QueryBus` (like any hexagonal adapter
//! dependency) and genuinely reads `req.ctx` inside its own `execute()` body for observability —
//! `HandlerContext` is available but the login check itself doesn't need it.
//!
//! (Formerly `AuthSvc` implementing `Service`, wrapped by a hand-composed `AuthHandler` —
//! `Service`/`ServiceRegistry` were removed as redundant with `Handler`/`HandlerRegistry`, see
//! issue #147. Collapsed into one `Handler` directly.)
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_query::{
    DirectQueryBus, Query, QueryBus, QueryDispatchRequest, QueryError, QueryExecuteRequest,
    QueryResultResponse,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;
use futures::future::BoxFuture;

#[derive(Debug, Clone)]
struct IsLoggedInRequest {
    session_token: String,
}
impl edge_application_base::Request for IsLoggedInRequest {}

#[derive(Debug, Clone, Copy)]
struct IsLoggedInResponse {
    logged_in: bool,
}
impl edge_application_base::Response for IsLoggedInResponse {}

/// The actual infra unit, reached only through `AuthHandler`'s own injected `QueryBus`.
struct LoginStatusQuery {
    session_token: String,
}

impl Query for LoginStatusQuery {
    type Result = bool;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<bool>, QueryError>> {
        let logged_in = !self.session_token.is_empty();
        Box::pin(async move { Ok(QueryResultResponse { result: logged_in }) })
    }
}

/// Holds its own injected `QueryBus`; genuinely reads `req.ctx` inside `execute()`.
struct AuthHandler {
    query_bus: Arc<dyn QueryBus<Result = bool>>,
}

#[async_trait]
impl Handler for AuthHandler {
    type Request = IsLoggedInRequest;
    type Response = IsLoggedInResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, IsLoggedInRequest>,
    ) -> Result<IsLoggedInResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "auth_handler".to_string(),
                message: format!("checking login for {:?}", req.req.session_token),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        let result = self
            .query_bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(LoginStatusQuery {
                    session_token: req.req.session_token,
                }),
            })
            .await
            .map_err(|e: QueryError| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(IsLoggedInResponse {
            logged_in: result.result,
        })
    }
}

fn build_ctx_and_run(
    handler: &AuthHandler,
    session_token: &str,
) -> Result<IsLoggedInResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: IsLoggedInRequest {
            session_token: session_token.to_string(),
        },
        ctx: &ctx,
    }))
}

fn build_handler() -> AuthHandler {
    let query_bus: Arc<dyn QueryBus<Result = bool>> = Arc::new(DirectQueryBus::<bool>::new());
    AuthHandler { query_bus }
}

/// @covers: Handler::execute
#[test]
fn test_execute_valid_token_returns_logged_in_true_happy() {
    let handler = build_handler();
    let response = build_ctx_and_run(&handler, "abc123").unwrap();
    assert!(response.logged_in);
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_token_returns_logged_in_false_error() {
    let handler = build_handler();
    let response = build_ctx_and_run(&handler, "").unwrap();
    assert!(!response.logged_in);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let handler = build_handler();
    let first = build_ctx_and_run(&handler, "session-a").unwrap();
    let second = build_ctx_and_run(&handler, "").unwrap();
    let third = build_ctx_and_run(&handler, "session-b").unwrap();
    assert!(first.logged_in);
    assert!(!second.logged_in);
    assert!(third.logged_in);
}
