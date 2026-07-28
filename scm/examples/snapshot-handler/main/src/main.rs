//! Runnable example: two `Handler`s sharing one constructor-injected
//! `SnapshotStore<AggregateId, Snap>`, with the per-call aggregate id/version carried through
//! `Self::Request` — the generic-per-type port wiring pattern `HandlerContext` structurally
//! can't hold. See issue #149.
//!
//! Unlike `saga`, `SnapshotStore::save`/`load` both take `&self` (interior mutability handled
//! internally by `MemorySnapshotStore`'s own `RwLock`) and are both async — no structural
//! blocker composing with `Handler::execute`, same shape as `repository`/`event`.
//!
//! `SnapshotPersistenceHandler::execute` genuinely reads `HandlerContext` for logging; the
//! actual save doesn't need it. Both handlers here share the same injected
//! `Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>` instance.
//!
//! Run with: `cargo run -p edge-application-snapshot-handler-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_application_snapshot::{MemorySnapshotStore, SnapshotStore};
use edge_security_runtime::SecurityContext;

use crate::api::{OrderSnap, SnapshotLookupRequest, SnapshotPersistenceRequest};
use crate::core::{SnapshotLookupHandler, SnapshotPersistenceHandler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== Handler + injected SnapshotStore<AggregateId, Snap> — constructor injection, Self::Request carries the version ===\n");

    let snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>> =
        Arc::new(MemorySnapshotStore::<OrderSnap>::new());
    let persistence_handler = SnapshotPersistenceHandler {
        snapshot_store: Arc::clone(&snapshot_store),
    };
    let lookup_handler = SnapshotLookupHandler {
        snapshot_store: Arc::clone(&snapshot_store),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] persistence_handler.execute(SnapshotPersistenceRequest {{ aggregate_id: \"order-1\", version: 3, total: 42 }})");
    let save_resp = persistence_handler
        .execute(HandlerExecutionRequest {
            req: SnapshotPersistenceRequest {
                aggregate_id: "order-1".to_string(),
                version: 3,
                total: 42,
            },
            ctx: &ctx,
        })
        .await
        .expect("save should succeed");
    println!("[3] SnapshotPersistenceHandler reports saved = {}\n", save_resp.saved);

    println!("[0] lookup_handler.execute(SnapshotLookupRequest {{ aggregate_id: \"order-1\" }})");
    let load_resp = lookup_handler
        .execute(HandlerExecutionRequest {
            req: SnapshotLookupRequest {
                aggregate_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("load should succeed");
    println!("[3] SnapshotLookupHandler returned {:?}\n", load_resp.snapshot);

    println!("[0] persistence_handler.execute(SnapshotPersistenceRequest {{ aggregate_id: \"order-2\", version: 0, total: 0 }}) — invalid version");
    let invalid = persistence_handler
        .execute(HandlerExecutionRequest {
            req: SnapshotPersistenceRequest {
                aggregate_id: "order-2".to_string(),
                version: 0,
                total: 0,
            },
            ctx: &ctx,
        })
        .await;
    match invalid {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] SnapshotPersistenceHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: SnapshotPersistenceHandler used ctx.observer for real per-request logging, but");
    println!("both handlers reached the actual store only through their own, independently");
    println!("injected SnapshotStore<String, OrderSnap> — never through HandlerContext, which");
    println!("structurally cannot hold a generic-per-type port. The store's own version-0 rejection");
    println!("propagated cleanly as a real HandlerError, not a fabricated one.");
}
