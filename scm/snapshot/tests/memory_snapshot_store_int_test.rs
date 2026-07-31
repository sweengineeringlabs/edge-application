//! Integration tests for `MemorySnapshotStore` — covers the type file directly.
// @allow: no_mocks_in_integration — MemorySnapshotStore is the production-shipped reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_snapshot::{
    MemorySnapshotStore, Snapshot, SnapshotAggregateIdRequest, SnapshotAggregateIdResponse,
    SnapshotError, SnapshotLoadRequest, SnapshotSaveRequest, SnapshotStore, SnapshotVersionRequest,
    SnapshotVersionResponse,
};
use futures::executor::block_on;

#[derive(Clone)]
struct OrderSnapshot {
    id: String,
    version: u64,
}
impl Snapshot for OrderSnapshot {
    type AggregateId = String;
    fn aggregate_id(
        &self,
        _req: SnapshotAggregateIdRequest,
    ) -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
        Ok(SnapshotAggregateIdResponse {
            aggregate_id: &self.id,
        })
    }
    fn version(
        &self,
        _req: SnapshotVersionRequest,
    ) -> Result<SnapshotVersionResponse, SnapshotError> {
        Ok(SnapshotVersionResponse {
            version: self.version,
        })
    }
}

fn order_snapshot(id: &str, v: u64) -> OrderSnapshot {
    OrderSnapshot {
        id: id.to_string(),
        version: v,
    }
}

/// @covers: MemorySnapshotStore::new — creates empty store
#[test]
fn test_new_creates_empty_store_happy() {
    let store = MemorySnapshotStore::<OrderSnapshot>::new();
    assert!(store.snapshots.read().unwrap().is_empty());
}

/// @covers: MemorySnapshotStore — load missing returns None
#[test]
fn test_load_missing_aggregate_returns_none_error() {
    let store = MemorySnapshotStore::<OrderSnapshot>::new();
    let id = "ghost".to_string();
    let result = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert!(result.is_none());
}

/// @covers: MemorySnapshotStore — isolates different aggregates
#[test]
fn test_load_different_aggregate_id_returns_none_edge() {
    let store = MemorySnapshotStore::<OrderSnapshot>::new();
    block_on(store.save(SnapshotSaveRequest {
        snapshot: order_snapshot("a", 1),
    }))
    .unwrap();
    let id = "b".to_string();
    let result = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert!(result.is_none());
}
