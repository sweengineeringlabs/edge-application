//! Runnable example: two `Handler`s sharing one constructor-injected `Projection<Event,
//! ReadModel>`, with the per-call events carried through `Self::Request` — the generic-per-type
//! port wiring pattern `HandlerContext` structurally can't hold. See issue #149.
//!
//! `Projection::apply`/`try_drain` take `&mut self` (same shape as `Saga::handle`), so this
//! wires `Handler` to the projection via `Arc<parking_lot::Mutex<P>>`, same as
//! `examples/saga-handler`. Unlike `saga`, there's no separate store trait with an
//! immutable-borrow-return problem — `Projection` is held directly.
//!
//! `CreditPostingHandler::execute` genuinely reads `HandlerContext` for logging. Both handlers
//! here share the same injected `Arc<Mutex<Balance>>` instance.
//!
//! Run with: `cargo run -p edge-application-projection-handler-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use parking_lot::Mutex;

use crate::api::{Balance, BalanceLookupRequest, CreditPostingRequest};
use crate::core::{BalanceLookupHandler, CreditPostingHandler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== Handler + injected Projection<Event, ReadModel> (Arc<Mutex<P>>) — constructor injection, Self::Request carries the events ===\n");

    let balance = Arc::new(Mutex::new(Balance::default()));
    let apply_handler = CreditPostingHandler {
        balance: Arc::clone(&balance),
    };
    let get_handler = BalanceLookupHandler {
        balance: Arc::clone(&balance),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] apply_handler.execute(CreditPostingRequest {{ amounts: [10, 20, 5] }})");
    let applied = apply_handler
        .execute(HandlerExecutionRequest {
            req: CreditPostingRequest {
                amounts: vec![10, 20, 5],
            },
            ctx: &ctx,
        })
        .await
        .expect("apply should succeed");
    println!("[3] CreditPostingHandler applied {} event(s)\n", applied.applied);

    println!("[0] get_handler.execute(BalanceLookupRequest)");
    let balance_resp = get_handler
        .execute(HandlerExecutionRequest {
            req: BalanceLookupRequest,
            ctx: &ctx,
        })
        .await
        .expect("read_model should succeed");
    println!("[3] BalanceLookupHandler reports total = {}\n", balance_resp.total);

    println!("[0] apply_handler.execute(CreditPostingRequest {{ amounts: [] }}) — empty batch");
    let empty = apply_handler
        .execute(HandlerExecutionRequest {
            req: CreditPostingRequest { amounts: vec![] },
            ctx: &ctx,
        })
        .await;
    match empty {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] CreditPostingHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: both handlers reached the same projection only through their own,");
    println!("independently injected Arc<Mutex<Balance>> — never through HandlerContext, which");
    println!("structurally cannot hold a generic-per-type port. The projection's own empty-batch");
    println!("rejection propagated cleanly as a real HandlerError.");
}
