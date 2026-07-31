//! API-layer type for the in-memory snapshot store.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::api::snapshot::traits::Snapshot;

/// An in-memory snapshot store — keeps the latest snapshot per aggregate.
///
/// A reference [`SnapshotStore`](crate::SnapshotStore) for development and
/// testing.  State lives in process memory and is lost when the process stops.
pub struct MemorySnapshotStore<S: Snapshot> {
    /// The underlying snapshot map — exposed for test-time inspection.
    pub snapshots: RwLock<HashMap<S::AggregateId, S>>,
}
