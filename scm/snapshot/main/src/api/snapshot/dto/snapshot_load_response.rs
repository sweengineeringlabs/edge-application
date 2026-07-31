//! [`SnapshotLoadResponse`] — wrapper for an optional loaded snapshot.

/// Result of [`SnapshotStore::load`](crate::api::SnapshotStore::load).
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SnapshotLoadResponse<S> {
    /// The latest snapshot for the requested aggregate, if any has been saved.
    pub snapshot: Option<S>,
}
