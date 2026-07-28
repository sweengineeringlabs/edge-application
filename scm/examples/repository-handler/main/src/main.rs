//! Runnable example: two `Handler`s sharing one constructor-injected `Repository<Entity, Id>`,
//! with the per-call entity id carried through `Self::Request` — the generic-per-type port
//! wiring pattern `HandlerContext` structurally can't hold, since a single context field can't
//! be "a repository," only "a repository of `Order`s." See issue #149.
//!
//! `OrderPersistenceHandler::execute` genuinely reads `HandlerContext` (it emits a log record
//! through `ctx.observer` on every call); the actual persistence doesn't need it — the
//! `Repository` it holds is its own collaborator, injected once at construction, exactly like
//! `AuthHandler`'s `QueryBus` in `examples/service-query`. Both handlers here share the same
//! injected `Arc<dyn Repository<...>>` instance, demonstrating the realistic case where one
//! repository backs multiple handlers.
//!
//! Run with: `cargo run -p edge-application-repository-handler-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_application_repository::{MemoryRepository, Repository};
use edge_security_runtime::SecurityContext;

use crate::api::{OrderLookupRequest, OrderPersistenceRequest};
use crate::core::{OrderLookupHandler, OrderPersistenceHandler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== Handler + injected Repository<Entity, Id> — constructor injection, Self::Request carries the id ===\n");

    let repository: Arc<dyn Repository<Entity = api::Order, Id = String>> =
        Arc::new(MemoryRepository::<api::Order, String>::new());
    let persistence_handler = OrderPersistenceHandler {
        repository: Arc::clone(&repository),
    };
    let lookup_handler = OrderLookupHandler {
        repository: Arc::clone(&repository),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] persistence_handler.execute(OrderPersistenceRequest {{ order_id: \"order-1\", item: \"widget\", quantity: 3 }})");
    let save_resp = persistence_handler
        .execute(HandlerExecutionRequest {
            req: OrderPersistenceRequest {
                order_id: "order-1".to_string(),
                item: "widget".to_string(),
                quantity: 3,
            },
            ctx: &ctx,
        })
        .await
        .expect("save should succeed");
    println!("[3] OrderPersistenceHandler reports saved = {}\n", save_resp.saved);

    println!("[0] lookup_handler.execute(OrderLookupRequest {{ order_id: \"order-1\" }})");
    let get_resp = lookup_handler
        .execute(HandlerExecutionRequest {
            req: OrderLookupRequest {
                order_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("get should succeed");
    println!("[3] OrderLookupHandler returned {:?}\n", get_resp.order);

    println!("[0] lookup_handler.execute(OrderLookupRequest {{ order_id: \"missing\" }})");
    let missing_resp = lookup_handler
        .execute(HandlerExecutionRequest {
            req: OrderLookupRequest {
                order_id: "missing".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("get should succeed even on a miss");
    println!("[3] OrderLookupHandler returned {:?}\n", missing_resp.order);

    println!("Conclusion: OrderPersistenceHandler used ctx.observer for real per-request logging, but");
    println!("both handlers reached the actual store only through their own, independently");
    println!("injected Repository<Order, String> — never through HandlerContext, which structurally");
    println!("cannot hold a generic-per-type port. Swap MemoryRepository for a real database-backed");
    println!("Repository impl and neither handler changes.");
}
