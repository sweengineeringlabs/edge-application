//! `OrderSnap` — the snapshot type stored in the injected `SnapshotStore<String, OrderSnap>`.

use edge_application_snapshot::{
    Snapshot, SnapshotAggregateIdRequest, SnapshotAggregateIdResponse, SnapshotError,
    SnapshotVersionRequest, SnapshotVersionResponse,
};

#[derive(Debug, Clone)]
pub struct OrderSnap {
    pub aggregate_id: String,
    pub version: u64,
    pub total: u64,
}

impl Snapshot for OrderSnap {
    type AggregateId = String;

    fn aggregate_id(
        &self,
        _req: SnapshotAggregateIdRequest,
    ) -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
        Ok(SnapshotAggregateIdResponse {
            aggregate_id: &self.aggregate_id,
        })
    }
    fn version(&self, _req: SnapshotVersionRequest) -> Result<SnapshotVersionResponse, SnapshotError> {
        Ok(SnapshotVersionResponse {
            version: self.version,
        })
    }
}
