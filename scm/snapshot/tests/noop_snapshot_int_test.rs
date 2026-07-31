#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_snapshot::{
    NoopSnapshot, Snapshot, SnapshotAggregateIdRequest, SnapshotVersionRequest,
};

#[test]
fn test_noop_snapshot_aggregate_id_returns_empty_string_happy() {
    let s = NoopSnapshot::default();
    assert_eq!(
        s.aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        ""
    );
}

#[test]
fn test_noop_snapshot_version_returns_zero_happy() {
    let s = NoopSnapshot::default();
    assert_eq!(s.version(SnapshotVersionRequest).unwrap().version, 0);
}

#[test]
fn test_noop_snapshot_clone_produces_equal_value_edge() {
    let s = NoopSnapshot::default();
    let cloned = s.clone();
    assert_eq!(
        s.aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        cloned
            .aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id
    );
    assert_eq!(
        s.version(SnapshotVersionRequest).unwrap().version,
        cloned.version(SnapshotVersionRequest).unwrap().version
    );
}
