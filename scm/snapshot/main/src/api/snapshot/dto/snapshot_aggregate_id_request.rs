//! [`SnapshotAggregateIdRequest`] — zero-sized marker for querying a snapshot's aggregate id.

/// Request to read the aggregate id a [`Snapshot`](crate::api::Snapshot) belongs to.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotAggregateIdRequest;
