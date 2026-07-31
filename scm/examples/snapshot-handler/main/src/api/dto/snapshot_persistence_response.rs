//! `SnapshotPersistenceResponse` — `SnapshotPersistenceHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct SnapshotPersistenceResponse {
    pub saved: bool,
}
impl edge_application_base::Response for SnapshotPersistenceResponse {}
