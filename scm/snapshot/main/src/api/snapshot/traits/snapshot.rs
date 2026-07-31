//! `Snapshot` — a point-in-time capture of aggregate state.

use std::hash::Hash;

use crate::api::snapshot::errors::SnapshotError;
use crate::api::snapshot::dto::{
    SnapshotAggregateIdRequest, SnapshotAggregateIdResponse, SnapshotVersionRequest,
    SnapshotVersionResponse,
};

/// A point-in-time capture of aggregate state used to short-circuit full event replay.
pub trait Snapshot: Send + Sync {
    /// The aggregate identity type this snapshot is keyed by.
    type AggregateId: Eq + Hash + Clone + Send + Sync;

    /// The aggregate id this snapshot belongs to.
    fn aggregate_id(
        &self,
        req: SnapshotAggregateIdRequest,
    ) -> Result<SnapshotAggregateIdResponse<'_, Self::AggregateId>, SnapshotError>;

    /// The event stream version at the time this snapshot was taken.
    fn version(
        &self,
        req: SnapshotVersionRequest,
    ) -> Result<SnapshotVersionResponse, SnapshotError>;
}
