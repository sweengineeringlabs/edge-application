//! Integration test: `SnapshotPersistenceHandler::execute()` genuinely reads `HandlerContext` for
//! real observability and, as part of the same call, persists through a real,
//! constructor-injected `SnapshotStore` — `Handler` -> `SnapshotStore`, connected and working, as
//! one observable call chain. Assertions verify the write landed in the shared store, and that
//! the store's own version-0 validation propagates as a genuine `HandlerError`, not a fabricated
//! one.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
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
struct SnapshotPersistenceRequest {
    aggregate_id: String,
    version: u64,
    total: u64,
}
impl edge_application_base::Request for SnapshotPersistenceRequest {}

#[derive(Debug, Clone, Copy)]
struct SnapshotPersistenceResponse {
    saved: bool,
}
impl edge_application_base::Response for SnapshotPersistenceResponse {}

/// Holds its own injected `SnapshotStore`; genuinely reads `req.ctx` inside `execute()`.
struct SnapshotPersistenceHandler {
    snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>,
}

#[async_trait]
impl Handler for SnapshotPersistenceHandler {
    type Request = SnapshotPersistenceRequest;
    type Response = SnapshotPersistenceResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SnapshotPersistenceRequest>,
    ) -> Result<SnapshotPersistenceResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "snapshot_persistence_handler".to_string(),
                message: format!("saving snapshot for {:?}", req.req.aggregate_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        self.snapshot_store
            .save(SnapshotSaveRequest {
                snapshot: OrderSnap {
                    aggregate_id: req.req.aggregate_id,
                    version: req.req.version,
                    total: req.req.total,
                },
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(SnapshotPersistenceResponse { saved: true })
    }
}

fn build_handler() -> (
    SnapshotPersistenceHandler,
    Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>>,
) {
    let snapshot_store: Arc<dyn SnapshotStore<AggregateId = String, Snap = OrderSnap>> =
        Arc::new(MemorySnapshotStore::<OrderSnap>::new());
    let handler = SnapshotPersistenceHandler {
        snapshot_store: Arc::clone(&snapshot_store),
    };
    (handler, snapshot_store)
}

fn run(
    handler: &SnapshotPersistenceHandler,
    aggregate_id: &str,
    version: u64,
    total: u64,
) -> Result<SnapshotPersistenceResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: SnapshotPersistenceRequest {
            aggregate_id: aggregate_id.to_string(),
            version,
            total,
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_valid_version_persists_in_store_happy() {
    let (handler, snapshot_store) = build_handler();
    let response = run(&handler, "order-1", 3, 42).unwrap();
    assert!(response.saved);

    let loaded = block_on(snapshot_store.load(SnapshotLoadRequest {
        id: &"order-1".to_string(),
    }))
    .unwrap()
    .snapshot;
    let snap = loaded.unwrap();
    assert_eq!(snap.version, 3);
    assert_eq!(snap.total, 42);
}

/// @covers: Handler::execute
#[test]
fn test_execute_version_zero_returns_execution_failed_error() {
    let (handler, _snapshot_store) = build_handler();
    let err = run(&handler, "order-2", 0, 0).unwrap_err();
    assert!(matches!(err, HandlerError::ExecutionFailed(_)));
}

/// @covers: Handler::execute
#[test]
fn test_execute_same_aggregate_twice_last_write_wins_edge() {
    let (handler, snapshot_store) = build_handler();
    run(&handler, "order-3", 1, 10).unwrap();
    run(&handler, "order-3", 2, 20).unwrap();

    let loaded = block_on(snapshot_store.load(SnapshotLoadRequest {
        id: &"order-3".to_string(),
    }))
    .unwrap()
    .snapshot;
    let snap = loaded.unwrap();
    assert_eq!(snap.version, 2);
    assert_eq!(snap.total, 20);
}
