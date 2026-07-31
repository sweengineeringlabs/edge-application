//! Runnable example: two `Handler`s sharing one constructor-injected saga instance, with the
//! per-call event data carried through `Self::Request` — the generic-per-type port wiring
//! pattern `HandlerContext` structurally can't hold. See issue #149.
//!
//! **A real structural finding, not a workaround:** `SagaStore::get<'a>(&'a self, ...) ->
//! Result<SagaGetResponse<'a, Self::SagaInstance>, SagaError>` returns an *immutable* borrow of
//! the stored saga, but `Saga::handle(&mut self, ...)` requires a *mutable* one. No amount of
//! external locking around the store fixes this — the trait method's own return type is the
//! blocker, not the caller's access pattern. `SagaStore` as currently shaped can register and
//! look sagas up (both fine, `is_complete` only needs `&self` too), but it cannot support
//! "retrieve a saga, then call `handle` on it" at all. So this example wires `Handler` directly
//! to a single `Saga` instance behind `Arc<parking_lot::Mutex<S>>` — bypassing `SagaStore`
//! entirely for the mutation path, because `SagaStore` structurally can't do it, not because
//! this example chose to skip it.
//!
//! `SagaEventHandler::execute` genuinely reads `HandlerContext` for logging; the saga
//! mutation itself doesn't need it. Both handlers here share the same injected
//! `Arc<Mutex<OrderSaga>>` instance.
//!
//! Run with: `cargo run -p edge-application-saga-handler-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use parking_lot::Mutex;

use crate::api::{OrderSaga, SagaCompletionRequest, SagaEventRequest};
use crate::core::{SagaCompletionHandler, SagaEventHandler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== Handler + injected Saga (Arc<Mutex<S>>) — constructor injection, Self::Request carries the event ===\n");

    let saga = Arc::new(Mutex::new(OrderSaga::default()));
    let handle_handler = SagaEventHandler {
        saga: Arc::clone(&saga),
    };
    let check_handler = SagaCompletionHandler {
        saga: Arc::clone(&saga),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] check_handler.execute(SagaCompletionRequest) — before any event");
    let before = check_handler
        .execute(HandlerExecutionRequest {
            req: SagaCompletionRequest,
            ctx: &ctx,
        })
        .await
        .expect("is_complete should succeed");
    println!("[3] SagaCompletionHandler reports complete = {}\n", before.complete);

    println!("[0] handle_handler.execute(SagaEventRequest {{ order_id: \"order-1\" }})");
    let handled = handle_handler
        .execute(HandlerExecutionRequest {
            req: SagaEventRequest {
                order_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("handle should succeed");
    println!(
        "[3] SagaEventHandler dispatched {} command(s)\n",
        handled.commands_dispatched
    );

    println!("[0] check_handler.execute(SagaCompletionRequest) — after the event");
    let after = check_handler
        .execute(HandlerExecutionRequest {
            req: SagaCompletionRequest,
            ctx: &ctx,
        })
        .await
        .expect("is_complete should succeed");
    println!("[3] SagaCompletionHandler reports complete = {}\n", after.complete);

    println!("Conclusion: both handlers reached the same saga only through their own,");
    println!("independently injected Arc<Mutex<OrderSaga>> — never through HandlerContext. This");
    println!("bypasses SagaStore entirely, because SagaStore::get()'s immutable-borrow return type");
    println!("cannot support the get-then-handle mutation path Saga::handle requires.");
}
