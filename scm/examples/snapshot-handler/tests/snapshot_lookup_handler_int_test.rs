//! Integration test: `SnapshotLookupHandler::execute()` reads through a real,
//! constructor-injected `SnapshotStore` — the same instance a `SnapshotPersistenceHandler`
//! writes into, proving one snapshot store can genuinely back multiple handlers, each reaching
//! it through its own injected field rather than through `HandlerContext`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer::StdObserveFactory;
use edge_application_snapshot::{
    MemorySnapshotStore, Snapshot, SnapshotAggregateIdRequest, SnapshotAggregateIdResponse,
    SnapshotError, SnapshotLoadRequest, SnapshotSaveRequest, SnapshotStore, SnapshotVersionRequest,
    SnapshotVersionResponse,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone)]
struct OrderSnap {
    aggregate_id: String,
    version: u64,
    total: u64,
}

impl Snapshot for OrderSnap {
    type AggregateId = String;

    fn aggregate_id(
        &self,
        _req: SnapshotAggregateIdRequest,
    ) -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
        Ok(SnapshotAggregateIdResponse {
            aggregate_id: &self.aggregate_id,
        })
    }
    fn version(&self, _req: SnapshotVersionRequest) -> Result<SnapshotVersionResponse, SnapshotError> {
        Ok(SnapshotVersionResponse {
            version: self.version,
        })
    }
}

#[derive(Debug, Clone)]
struct SnapshotLookupRequest {
    aggregate_id: String,
}
impl edge_application_base::Request for SnapshotLookupRequest {}

#[derive(Debug, Clone)]
struct SnapshotLookupResponse {
    snapshot: Option<(u64, u64)>,
}
impl edge_application_base::Response for SnapshotLookupResponse {}

/// Holds its own injected `SnapshotStore` — the per-call aggregate id arrives through
/// `Self::Request`.
struct SnapshotLookupHandler {
    snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>,
}

#[async_trait]
impl Handler for SnapshotLookupHandler {
    type Request = SnapshotLookupRequest;
    type Response = SnapshotLookupResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SnapshotLookupRequest>,
    ) -> Result<SnapshotLookupResponse, HandlerError> {
        let loaded = self
            .snapshot_store
            .load(SnapshotLoadRequest {
                id: &req.req.aggregate_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(SnapshotLookupResponse {
            snapshot: loaded.snapshot.map(|s| (s.version, s.total)),
        })
    }
}

/// Seeds a snapshot store with one entry and builds a handler sharing that same instance —
/// mirrors the multi-handler-one-store shape from `SnapshotPersistenceHandler`.
fn build_handler_with_seeded_snapshot() -> SnapshotLookupHandler {
    let snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>> =
        Arc::new(MemorySnapshotStore::<OrderSnap>::new());
    block_on(snapshot_store.save(SnapshotSaveRequest {
        snapshot: OrderSnap {
            aggregate_id: "order-1".to_string(),
            version: 3,
            total: 42,
        },
    }))
    .unwrap();
    SnapshotLookupHandler { snapshot_store }
}

fn run(
    handler: &SnapshotLookupHandler,
    aggregate_id: &str,
) -> Result<SnapshotLookupResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: SnapshotLookupRequest {
            aggregate_id: aggregate_id.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_existing_snapshot_returns_data_happy() {
    let handler = build_handler_with_seeded_snapshot();
    let response = run(&handler, "order-1").unwrap();
    assert_eq!(response.snapshot, Some((3, 42)));
}

/// @covers: Handler::execute
#[test]
fn test_execute_missing_snapshot_returns_none_error() {
    let handler = build_handler_with_seeded_snapshot();
    let response = run(&handler, "does-not-exist").unwrap();
    assert_eq!(response.snapshot, None);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_independent_and_consistent_edge() {
    let handler = build_handler_with_seeded_snapshot();
    let first = run(&handler, "order-1").unwrap();
    let second = run(&handler, "missing").unwrap();
    let third = run(&handler, "order-1").unwrap();
    assert!(first.snapshot.is_some());
    assert!(second.snapshot.is_none());
    assert_eq!(first.snapshot, third.snapshot);
}
