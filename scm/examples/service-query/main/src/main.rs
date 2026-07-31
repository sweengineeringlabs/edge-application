//! Runnable example: a `Handler` reaching real infra work through a `Query`, using ordinary
//! constructor injection — no trait changes anywhere, no deviation from the architecture.
//!
//! `AuthHandler::execute` genuinely reads `HandlerContext` (it emits a log record through
//! `ctx.observer` on every call), but the actual login check doesn't need it: the `QueryBus` it
//! rides is its own collaborator, injected once at construction, exactly like any other
//! hexagonal adapter dependency. Swap `LoginStatusQuery`'s body for a real
//! session-store/auth-backend call when that's ready, and `AuthHandler` itself never changes.
//!
//! (Formerly demonstrated this pattern through a `Service` wrapped by a hand-composed `Handler`
//! — `Service`/`ServiceRegistry` were removed as redundant with `Handler`/`HandlerRegistry`, see
//! issue #147. `Handler` alone now carries the same "constructor-injected collaborator, context
//! available but not required" story directly.)
//!
//! Run with: `cargo run -p edge-application-service-query-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_application_query::{DirectQueryBus, QueryBus};
use edge_security_runtime::SecurityContext;

use crate::api::IsLoggedInRequest;
use crate::core::AuthHandler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== AuthHandler riding infra via Query — constructor injection, no trait changes ===\n");

    let query_bus: Arc<dyn QueryBus<Result = bool>> = Arc::new(DirectQueryBus::<bool>::new());
    let auth_handler = AuthHandler { query_bus };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    for session_token in ["abc123", ""] {
        println!("[0] auth_handler.execute(IsLoggedInRequest {{ session_token: {session_token:?} }})");
        match auth_handler
            .execute(HandlerExecutionRequest {
                req: IsLoggedInRequest {
                    session_token: session_token.to_string(),
                },
                ctx: &ctx,
            })
            .await
        {
            Ok(response) => println!("[4] AuthHandler reports logged_in = {}\n", response.logged_in),
            Err(e) => println!("[4] AuthHandler returned an error: {e}\n"),
        }
    }

    println!("Conclusion: AuthHandler used ctx.observer for real per-request logging, but its own,");
    println!("independently injected QueryBus to reach the actual infra (LoginStatusQuery) never");
    println!("needed HandlerContext at all. Swap LoginStatusQuery's body for a real session-store");
    println!("call and AuthHandler itself never changes. No trait changes anywhere in");
    println!("Handler/Query/QueryBus were needed.");
}
