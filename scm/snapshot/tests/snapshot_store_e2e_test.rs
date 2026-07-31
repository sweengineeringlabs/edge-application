//! SAF facade tests — `SnapshotStore` trait via `MemorySnapshotStore`.
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

/// @covers: SnapshotStore::save + load — round-trip
#[test]
fn test_save_then_load_returns_saved_snapshot_happy() {
    let store: MemorySnapshotStore<OrderSnapshot> = MemorySnapshotStore::new();
    block_on(store.save(SnapshotSaveRequest {
        snapshot: order_snapshot("a1", 5),
    }))
    .unwrap();
    let id = "a1".to_string();
    let loaded = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert_eq!(loaded.map(|s| s.version), Some(5));
}

/// @covers: SnapshotStore::save — version zero is rejected
#[test]
fn test_save_version_zero_returns_invalid_version_error() {
    let store: MemorySnapshotStore<OrderSnapshot> = MemorySnapshotStore::new();
    let err = block_on(store.save(SnapshotSaveRequest {
        snapshot: order_snapshot("a1", 0),
    }))
    .unwrap_err();
    assert_eq!(
        err,
        SnapshotError::InvalidVersion {
            aggregate_id: "a1".into(),
            version: 0
        }
    );
}

/// @covers: SnapshotStore — save overwrites earlier snapshot
#[test]
fn test_save_overwrites_earlier_snapshot_edge() {
    let store: MemorySnapshotStore<OrderSnapshot> = MemorySnapshotStore::new();
    block_on(store.save(SnapshotSaveRequest {
        snapshot: order_snapshot("a1", 3),
    }))
    .unwrap();
    block_on(store.save(SnapshotSaveRequest {
        snapshot: order_snapshot("a1", 8),
    }))
    .unwrap();
    let id = "a1".to_string();
    let loaded = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert_eq!(loaded.map(|s| s.version), Some(8));
}

/// @covers: SnapshotStore::load — returns saved snapshot
#[test]
fn test_load_saved_snapshot_returns_some_happy() {
    let store: MemorySnapshotStore<OrderSnapshot> = MemorySnapshotStore::new();
    block_on(store.save(SnapshotSaveRequest {
        snapshot: order_snapshot("b1", 10),
    }))
    .unwrap();
    let id = "b1".to_string();
    let result = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert!(result.is_some());
}

/// @covers: SnapshotStore::load — missing aggregate returns None
#[test]
fn test_load_missing_aggregate_returns_none_error() {
    let store: MemorySnapshotStore<OrderSnapshot> = MemorySnapshotStore::new();
    let id = "missing".to_string();
    let result = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert!(result.is_none());
}

/// @covers: SnapshotStore::load — isolates different aggregate ids
#[test]
fn test_load_different_aggregate_id_returns_none_edge() {
    let store: MemorySnapshotStore<OrderSnapshot> = MemorySnapshotStore::new();
    block_on(store.save(SnapshotSaveRequest {
        snapshot: order_snapshot("c1", 2),
    }))
    .unwrap();
    let id = "c2".to_string();
    let result = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert!(result.is_none());
}
