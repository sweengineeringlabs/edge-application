//! `SnapshotLookupRequest` — `SnapshotLookupHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct SnapshotLookupRequest {
    pub aggregate_id: String,
}
impl edge_application_base::Request for SnapshotLookupRequest {}
