//! `SnapshotLookupResponse` — `SnapshotLookupHandler`'s `Self::Response`.

#[derive(Debug, Clone)]
pub struct SnapshotLookupResponse {
    pub snapshot: Option<(u64, u64)>,
}
impl edge_application_base::Response for SnapshotLookupResponse {}
