//! Layer-level coverage for `api/snapshot/dto/*.rs` request/response types.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_snapshot::{
    SnapshotAggregateIdRequest, SnapshotAggregateIdResponse, SnapshotLoadRequest,
    SnapshotLoadResponse, SnapshotSaveRequest, SnapshotVersionRequest, SnapshotVersionResponse,
};

/// @covers: SnapshotAggregateIdRequest
#[test]
fn test_snapshot_aggregate_id_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SnapshotAggregateIdRequest>(), 0);
    let _ = SnapshotAggregateIdRequest;
}

/// @covers: SnapshotAggregateIdResponse
#[test]
fn test_snapshot_aggregate_id_response_holds_id_happy() {
    let id = 42u32;
    let resp = SnapshotAggregateIdResponse { aggregate_id: &id };
    assert_eq!(*resp.aggregate_id, 42);
}

/// @covers: SnapshotLoadRequest
#[test]
fn test_snapshot_load_request_holds_id_happy() {
    let id = "agg-1".to_string();
    let req = SnapshotLoadRequest { id: &id };
    assert_eq!(req.id, "agg-1");
}

/// @covers: SnapshotLoadResponse
#[test]
fn test_snapshot_load_response_holds_some_happy() {
    let resp = SnapshotLoadResponse {
        snapshot: Some(7u32),
    };
    assert_eq!(resp.snapshot, Some(7));
}

/// @covers: SnapshotLoadResponse
#[test]
fn test_snapshot_load_response_none_edge() {
    let resp: SnapshotLoadResponse<u32> = SnapshotLoadResponse { snapshot: None };
    assert_eq!(resp.snapshot, None);
}

/// @covers: SnapshotSaveRequest
#[test]
fn test_snapshot_save_request_holds_snapshot_happy() {
    let req = SnapshotSaveRequest {
        snapshot: "my-snapshot".to_string(),
    };
    assert_eq!(req.snapshot, "my-snapshot");
}

/// @covers: SnapshotVersionRequest
#[test]
fn test_snapshot_version_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SnapshotVersionRequest>(), 0);
    let _ = SnapshotVersionRequest;
}

/// @covers: SnapshotVersionResponse
#[test]
fn test_snapshot_version_response_holds_version_happy() {
    let resp = SnapshotVersionResponse { version: 5 };
    assert_eq!(resp.version, 5);
}

/// @covers: SnapshotVersionResponse
#[test]
fn test_snapshot_version_response_zero_edge() {
    let resp = SnapshotVersionResponse { version: 0 };
    assert_eq!(resp.version, 0);
}
