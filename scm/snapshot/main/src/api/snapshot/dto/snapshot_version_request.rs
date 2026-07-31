//! [`SnapshotVersionRequest`] — zero-sized marker for querying a snapshot's version.

/// Request to read the event stream version a [`Snapshot`](crate::api::Snapshot) was taken at.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotVersionRequest;
