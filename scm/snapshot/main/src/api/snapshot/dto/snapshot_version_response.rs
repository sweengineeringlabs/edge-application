//! [`SnapshotVersionResponse`] — wrapper for a snapshot's event stream version.

/// Result of [`Snapshot::version`](crate::api::Snapshot::version).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotVersionResponse {
    /// The event stream version at the time this snapshot was taken.
    pub version: u64,
}
