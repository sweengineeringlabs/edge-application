//! Runnable example: two `Handler`s sharing one constructor-injected `EventStore<Event>`, with
//! the per-call aggregate id and event payload carried through `Self::Request` — the
//! generic-per-type port wiring pattern `HandlerContext` structurally can't hold, since a single
//! context field can't be "an event store," only "an event store of `OrderCreated` events." See
//! issue #149.
//!
//! `OrderCreationHandler::execute` genuinely reads `HandlerContext` (it emits a log record
//! through `ctx.observer` on every call); the actual append doesn't need it — the `EventStore` it
//! holds is its own collaborator, injected once at construction, same shape as
//! `OrderPersistenceHandler`'s `Repository` in `examples/repository-handler`. Both handlers here
//! share the same injected `Arc<dyn EventStore<Event = OrderCreated>>` instance.
//!
//! Run with: `cargo run -p edge-application-event-store-handler-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_event::{EventStore, MemoryEventStore};
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

use crate::api::{OrderCreated, OrderCreationRequest, OrderHistoryRequest};
use crate::core::{OrderCreationHandler, OrderHistoryHandler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== Handler + injected EventStore<Event> — constructor injection, Self::Request carries the aggregate id/payload ===\n");

    let event_store: Arc<dyn EventStore<Event = OrderCreated>> =
        Arc::new(MemoryEventStore::<OrderCreated>::new());
    let record_handler = OrderCreationHandler {
        event_store: Arc::clone(&event_store),
    };
    let history_handler = OrderHistoryHandler {
        event_store: Arc::clone(&event_store),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] record_handler.execute(OrderCreationRequest {{ order_id: \"order-1\", item: \"widget\" }})");
    let record_resp = record_handler
        .execute(HandlerExecutionRequest {
            req: OrderCreationRequest {
                order_id: "order-1".to_string(),
                item: "widget".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("append should succeed");
    println!(
        "[3] OrderCreationHandler reports sequence = {}\n",
        record_resp.sequence
    );

    println!("[0] record_handler.execute(OrderCreationRequest {{ order_id: \"order-1\", item: \"gadget\" }})");
    let record_resp_2 = record_handler
        .execute(HandlerExecutionRequest {
            req: OrderCreationRequest {
                order_id: "order-1".to_string(),
                item: "gadget".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("append should succeed");
    println!(
        "[3] OrderCreationHandler reports sequence = {}\n",
        record_resp_2.sequence
    );

    println!("[0] history_handler.execute(OrderHistoryRequest {{ order_id: \"order-1\" }})");
    let history_resp = history_handler
        .execute(HandlerExecutionRequest {
            req: OrderHistoryRequest {
                order_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("load should succeed");
    println!(
        "[3] OrderHistoryHandler returned {:?}\n",
        history_resp.items
    );

    println!("Conclusion: OrderCreationHandler used ctx.observer for real per-request logging,");
    println!("but both handlers reached the actual event stream only through their own,");
    println!("independently injected EventStore<OrderCreated> — never through HandlerContext, which");
    println!("structurally cannot hold a generic-per-type port. Swap MemoryEventStore for a real");
    println!("database-backed EventStore impl and neither handler changes.");
}
