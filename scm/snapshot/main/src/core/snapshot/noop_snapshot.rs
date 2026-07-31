//! `Snapshot` impl for [`NoopSnapshot`].

use crate::api::NoopSnapshot;
use crate::api::Snapshot;
use crate::api::SnapshotError;
use crate::api::{
    SnapshotAggregateIdRequest, SnapshotAggregateIdResponse, SnapshotVersionRequest,
    SnapshotVersionResponse,
};

impl Snapshot for NoopSnapshot {
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
