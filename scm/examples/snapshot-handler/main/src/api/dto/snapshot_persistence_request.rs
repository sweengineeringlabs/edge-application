//! `SnapshotPersistenceRequest` — `SnapshotPersistenceHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct SnapshotPersistenceRequest {
    pub aggregate_id: String,
    pub version: u64,
    pub total: u64,
}
impl edge_application_base::Request for SnapshotPersistenceRequest {}
