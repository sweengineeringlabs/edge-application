//! `SnapshotStore` — persists and retrieves [`Snapshot`]s.

use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;

use crate::api::snapshot::errors::SnapshotError;
use crate::api::snapshot::traits::Snapshot;
use crate::api::snapshot::dto::{SnapshotLoadRequest, SnapshotLoadResponse, SnapshotSaveRequest};

/// Stores and retrieves [`Snapshot`]s keyed by aggregate id.
pub trait SnapshotStore: Send + Sync {
    /// The aggregate identity type snapshots are keyed by.
    type AggregateId: Eq + Hash + Clone + Send + Sync;

    /// The snapshot type stored.
    type Snap: Snapshot<AggregateId = Self::AggregateId>;

    /// Persist a snapshot, replacing any earlier snapshot for the same aggregate.
    fn save(
        &self,
        req: SnapshotSaveRequest<Self::Snap>,
    ) -> Pin<Box<dyn Future<Output = Result<(), SnapshotError>> + Send + '_>>;

    /// Load the latest snapshot for `id`, or `None` if none has been saved.
    fn load<'a>(
        &'a self,
        req: SnapshotLoadRequest<'a, Self::AggregateId>,
    ) -> Pin<Box<dyn Future<Output = Result<SnapshotLoadResponse<Self::Snap>, SnapshotError>> + Send + 'a>>;
}
