//! Runnable example: two `Handler`s sharing one constructor-injected `Registry<Value>`, with
//! the per-call lookup id carried through `Self::Request` — the generic-per-type port wiring
//! pattern `HandlerContext` structurally can't hold. This is the last of the seven applicable
//! ports in issue #149.
//!
//! `Registry`'s own doc comment states every method takes `&self` and is concurrent by design
//! (`MemoryRegistry` handles interior mutability internally via `RwLock`) — the simplest port in
//! this series alongside `policy`, plain `Arc<dyn Registry<...>>` injection, no lock needed.
//!
//! `GreeterRegistrationHandler::execute` genuinely reads `HandlerContext` for logging. Both
//! handlers here share the same injected `Arc<dyn Registry<Value = dyn Greeter>>` instance.
//!
//! Run with: `cargo run -p edge-application-registry-handler-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_application_registry::{MemoryRegistry, Registry};
use edge_security_runtime::SecurityContext;

use crate::api::{Greeter, GreeterRegistrationRequest, GreeterResolutionRequest};
use crate::core::{GreeterRegistrationHandler, GreeterResolutionHandler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== Handler + injected Registry<Value> — constructor injection, Self::Request carries the id ===\n");

    let registry: Arc<dyn Registry<Value = dyn Greeter>> = Arc::new(MemoryRegistry::<dyn Greeter>::new());
    let register_handler = GreeterRegistrationHandler {
        registry: Arc::clone(&registry),
    };
    let resolve_handler = GreeterResolutionHandler {
        registry: Arc::clone(&registry),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] register_handler.execute(GreeterRegistrationRequest {{ id: \"es-greeter\", language: \"es\" }})");
    let register_resp = register_handler
        .execute(HandlerExecutionRequest {
            req: GreeterRegistrationRequest {
                id: "es-greeter".to_string(),
                language: "es".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("register should succeed");
    println!("[3] GreeterRegistrationHandler reports registered = {}\n", register_resp.registered);

    println!("[0] resolve_handler.execute(GreeterResolutionRequest {{ id: \"es-greeter\", name: \"Ada\" }})");
    let resolve_resp = resolve_handler
        .execute(HandlerExecutionRequest {
            req: GreeterResolutionRequest {
                id: "es-greeter".to_string(),
                name: "Ada".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("resolve should succeed");
    println!("[3] GreeterResolutionHandler returned {:?}\n", resolve_resp.greeting);

    println!("[0] register_handler.execute(GreeterRegistrationRequest {{ id: \"es-greeter\", language: \"en\" }}) — duplicate id");
    let duplicate = register_handler
        .execute(HandlerExecutionRequest {
            req: GreeterRegistrationRequest {
                id: "es-greeter".to_string(),
                language: "en".to_string(),
            },
            ctx: &ctx,
        })
        .await;
    match duplicate {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] GreeterRegistrationHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: GreeterRegistrationHandler used ctx.observer for real per-request logging,");
    println!("but both handlers reached the actual registry only through their own, independently");
    println!("injected Registry<dyn Greeter> — never through HandlerContext, which structurally");
    println!("cannot hold a generic-per-type port. The registry's own duplicate-id rejection");
    println!("propagated cleanly as a real HandlerError. This completes all seven applicable ports.");
}
